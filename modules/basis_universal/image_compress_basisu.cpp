/**************************************************************************/
/*  image_compress_basisu.cpp                                             */
/**************************************************************************/
/*                         This file is part of:                          */
/*                             Nebula Engine                              */
/*                    https://github.com/cruglet/nebula                   */
/**************************************************************************/
/* Copyright (c) 2024-present Nebula Engine contributors                  */
/* Copyright (c) 2014-present Godot Engine contributors (see AUTHORS.md). */
/*                                                                        */
/* Permission is hereby granted, free of charge, to any person obtaining  */
/* a copy of this software and associated documentation files (the        */
/* "Software"), to deal in the Software without restriction, including    */
/* without limitation the rights to use, copy, modify, merge, publish,    */
/* distribute, sublicense, and/or sell copies of the Software, and to     */
/* permit persons to whom the Software is furnished to do so, subject to  */
/* the following conditions:                                              */
/*                                                                        */
/* The above copyright notice and this permission notice shall be         */
/* included in all copies or substantial portions of the Software.        */
/*                                                                        */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,        */
/* EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF     */
/* MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. */
/* IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY   */
/* CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,   */
/* TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE      */
/* SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.                 */
/**************************************************************************/

#include "image_compress_basisu.h"

#include "servers/rendering_server.h"

#include <transcoder/basisu_transcoder.h>
#ifdef TOOLS_ENABLED
#include <encoder/basisu_comp.h>
#endif

void basis_universal_init() {
#ifdef TOOLS_ENABLED
	basisu::basisu_encoder_init();
#endif

	basist::basisu_transcoder_init();
}

#ifdef TOOLS_ENABLED
Vector<uint8_t> basis_universal_packer(const Ref<Image> &p_image, Image::UsedChannels p_channels) {
	Ref<Image> image = p_image->duplicate();
	image->convert(Image::FORMAT_RGBA8);

	basisu::basis_compressor_params params;

	params.m_uastc = true;
	params.m_quality_level = basisu::BASISU_QUALITY_MIN;
	params.m_pack_uastc_flags &= ~basisu::cPackUASTCLevelMask;
	params.m_pack_uastc_flags |= basisu::cPackUASTCLevelFastest;

	params.m_rdo_uastc = 0.0f;
	params.m_rdo_uastc_quality_scalar = 0.0f;
	params.m_rdo_uastc_dict_size = 1024;

	params.m_mip_fast = true;
	params.m_multithreading = true;
	params.m_check_for_alpha = false;

	if (!OS::get_singleton()->is_stdout_verbose()) {
		params.m_print_stats = false;
		params.m_compute_stats = false;
		params.m_status_output = false;
	}

	basisu::job_pool job_pool(OS::get_singleton()->get_processor_count());
	params.m_pJob_pool = &job_pool;

	BasisDecompressFormat decompress_format = BASIS_DECOMPRESS_RG;
	switch (p_channels) {
		case Image::USED_CHANNELS_L: {
			decompress_format = BASIS_DECOMPRESS_RGB;
		} break;
		case Image::USED_CHANNELS_LA: {
			params.m_force_alpha = true;
			decompress_format = BASIS_DECOMPRESS_RGBA;
		} break;
		case Image::USED_CHANNELS_R: {
			decompress_format = BASIS_DECOMPRESS_RGB;
		} break;
		case Image::USED_CHANNELS_RG: {
			// Currently RG textures are compressed as DXT5/ETC2_RGBA8 with a RA -> RG swizzle,
			// as BasisUniversal didn't use to support ETC2_RG11 transcoding.
			params.m_force_alpha = true;
			image->convert_rg_to_ra_rgba8();
			decompress_format = BASIS_DECOMPRESS_RG_AS_RA;
		} break;
		case Image::USED_CHANNELS_RGB: {
			decompress_format = BASIS_DECOMPRESS_RGB;
		} break;
		case Image::USED_CHANNELS_RGBA: {
			params.m_force_alpha = true;
			decompress_format = BASIS_DECOMPRESS_RGBA;
		} break;
	}

	// Copy the source image data with mipmaps into BasisU.
	{
		const int orig_width = image->get_width();
		const int orig_height = image->get_height();

		bool is_res_div_4 = (orig_width % 4 == 0) && (orig_height % 4 == 0);

		// Image's resolution rounded up to the nearest values divisible by 4.
		int next_width = orig_width <= 2 ? orig_width : (orig_width + 3) & ~3;
		int next_height = orig_height <= 2 ? orig_height : (orig_height + 3) & ~3;

		Vector<uint8_t> image_data = image->get_data();
		basisu::vector<basisu::image> basisu_mipmaps;

		// Buffer for storing padded mipmap data.
		Vector<uint32_t> mip_data_padded;

		for (int32_t i = 0; i <= image->get_mipmap_count(); i++) {
			int64_t ofs, size;
			int width, height;
			image->get_mipmap_offset_size_and_dimensions(i, ofs, size, width, height);

			const uint8_t *image_mip_data = image_data.ptr() + ofs;

			// Pad the mipmap's data if its resolution isn't divisible by 4.
			if (image->has_mipmaps() && !is_res_div_4 && (width > 2 && height > 2) && (width != next_width || height != next_height)) {
				// Source mip's data interpreted as 32-bit RGBA blocks to help with copying pixel data.
				const uint32_t *mip_src_data = reinterpret_cast<const uint32_t *>(image_mip_data);

				// Reserve space in the padded buffer.
				mip_data_padded.resize(next_width * next_height);
				uint32_t *data_padded_ptr = mip_data_padded.ptrw();

				// Pad mipmap to the nearest block by smearing.
				int x = 0, y = 0;
				for (y = 0; y < height; y++) {
					for (x = 0; x < width; x++) {
						data_padded_ptr[next_width * y + x] = mip_src_data[width * y + x];
					}

					// First, smear in x.
					for (; x < next_width; x++) {
						data_padded_ptr[next_width * y + x] = data_padded_ptr[next_width * y + x - 1];
					}
				}

				// Then, smear in y.
				for (; y < next_height; y++) {
					for (x = 0; x < next_width; x++) {
						data_padded_ptr[next_width * y + x] = data_padded_ptr[next_width * y + x - next_width];
					}
				}

				// Override the image_mip_data pointer with our temporary Vector.
				image_mip_data = reinterpret_cast<const uint8_t *>(mip_data_padded.ptr());

				// Override the mipmap's properties.
				width = next_width;
				height = next_height;
				size = mip_data_padded.size() * 4;
			}

			// Get the next mipmap's resolution.
			next_width /= 2;
			next_height /= 2;

			// Copy the source mipmap's data to a BasisU image.
			basisu::image basisu_image(width, height);
			memcpy(basisu_image.get_ptr(), image_mip_data, size);

			if (i == 0) {
				params.m_source_images.push_back(basisu_image);
			} else {
				basisu_mipmaps.push_back(basisu_image);
			}
		}

		params.m_source_mipmap_images.push_back(basisu_mipmaps);
	}

	// Encode the image data.
	Vector<uint8_t> basisu_data;

	basisu::basis_compressor compressor;
	compressor.init(params);

	int basisu_err = compressor.process();
	ERR_FAIL_COND_V(basisu_err != basisu::basis_compressor::cECSuccess, basisu_data);

	const basisu::uint8_vec &basisu_out = compressor.get_output_basis_file();
	basisu_data.resize(basisu_out.size() + 4);

	// Copy the encoded data to the buffer.
	{
		uint8_t *wb = basisu_data.ptrw();
		*(uint32_t *)wb = decompress_format;

		memcpy(wb + 4, basisu_out.get_ptr(), basisu_out.size());
	}

	return basisu_data;
}
#endif // TOOLS_ENABLED

Ref<Image> basis_universal_unpacker_ptr(const uint8_t *p_data, int p_size) {
	Ref<Image> image;
	ERR_FAIL_NULL_V_MSG(p_data, image, "Cannot unpack invalid BasisUniversal data.");

	const uint8_t *src_ptr = p_data;
	int src_size = p_size;

	basist::transcoder_texture_format basisu_format = basist::transcoder_texture_format::cTFTotalTextureFormats;
	Image::Format image_format = Image::FORMAT_MAX;

	// Get supported compression formats.
	bool bptc_supported = RS::get_singleton()->has_os_feature("bptc");
	bool astc_supported = RS::get_singleton()->has_os_feature("astc");
	bool s3tc_supported = RS::get_singleton()->has_os_feature("s3tc");
	bool etc2_supported = RS::get_singleton()->has_os_feature("etc2");

	bool needs_ra_rg_swap = false;

	switch (*(uint32_t *)(src_ptr)) {
		case BASIS_DECOMPRESS_RG: {
			// RGTC transcoding is currently performed with RG_AS_RA, fail.
			ERR_FAIL_V(image);
		} break;
		case BASIS_DECOMPRESS_RGB: {
			if (bptc_supported) {
				basisu_format = basist::transcoder_texture_format::cTFBC7_M6_OPAQUE_ONLY;
				image_format = Image::FORMAT_BPTC_RGBA;
			} else if (astc_supported) {
				basisu_format = basist::transcoder_texture_format::cTFASTC_4x4_RGBA;
				image_format = Image::FORMAT_ASTC_4x4;
			} else if (s3tc_supported) {
				basisu_format = basist::transcoder_texture_format::cTFBC1;
				image_format = Image::FORMAT_DXT1;
			} else if (etc2_supported) {
				basisu_format = basist::transcoder_texture_format::cTFETC1;
				image_format = Image::FORMAT_ETC2_RGB8;
			} else {
				// No supported VRAM compression formats, decompress.
				basisu_format = basist::transcoder_texture_format::cTFRGBA32;
				image_format = Image::FORMAT_RGBA8;
			}

		} break;
		case BASIS_DECOMPRESS_RGBA: {
			if (bptc_supported) {
				basisu_format = basist::transcoder_texture_format::cTFBC7_M5;
				image_format = Image::FORMAT_BPTC_RGBA;
			} else if (astc_supported) {
				basisu_format = basist::transcoder_texture_format::cTFASTC_4x4_RGBA;
				image_format = Image::FORMAT_ASTC_4x4;
			} else if (s3tc_supported) {
				basisu_format = basist::transcoder_texture_format::cTFBC3;
				image_format = Image::FORMAT_DXT5;
			} else if (etc2_supported) {
				basisu_format = basist::transcoder_texture_format::cTFETC2;
				image_format = Image::FORMAT_ETC2_RGBA8;
			} else {
				// No supported VRAM compression formats, decompress.
				basisu_format = basist::transcoder_texture_format::cTFRGBA32;
				image_format = Image::FORMAT_RGBA8;
			}
		} break;
		case BASIS_DECOMPRESS_RG_AS_RA: {
			if (s3tc_supported) {
				basisu_format = basist::transcoder_texture_format::cTFBC3;
				image_format = Image::FORMAT_DXT5_RA_AS_RG;
			} else if (etc2_supported) {
				basisu_format = basist::transcoder_texture_format::cTFETC2;
				image_format = Image::FORMAT_ETC2_RA_AS_RG;
			} else {
				// No supported VRAM compression formats, decompress.
				basisu_format = basist::transcoder_texture_format::cTFRGBA32;
				image_format = Image::FORMAT_RGBA8;
				needs_ra_rg_swap = true;
			}
		} break;
	}

	src_ptr += 4;
	src_size -= 4;

	basist::basisu_transcoder transcoder;
	ERR_FAIL_COND_V(!transcoder.validate_header(src_ptr, src_size), image);

	transcoder.start_transcoding(src_ptr, src_size);

	basist::basisu_image_info basisu_info;
	transcoder.get_image_info(src_ptr, src_size, basisu_info, 0);

	// Create the buffer for transcoded/decompressed data.
	Vector<uint8_t> out_data;
	out_data.resize(Image::get_image_data_size(basisu_info.m_width, basisu_info.m_height, image_format, basisu_info.m_total_levels > 1));

	uint8_t *dst = out_data.ptrw();
	memset(dst, 0, out_data.size());

	for (uint32_t i = 0; i < basisu_info.m_total_levels; i++) {
		basist::basisu_image_level_info basisu_level;
		transcoder.get_image_level_info(src_ptr, src_size, basisu_level, 0, i);

		uint32_t mip_block_or_pixel_count = Image::is_format_compressed(image_format) ? basisu_level.m_total_blocks : basisu_level.m_orig_width * basisu_level.m_orig_height;
		int64_t ofs = Image::get_image_mipmap_offset(basisu_info.m_width, basisu_info.m_height, image_format, i);

		bool result = transcoder.transcode_image_level(src_ptr, src_size, 0, i, dst + ofs, mip_block_or_pixel_count, basisu_format);

		if (!result) {
			print_line(vformat("BasisUniversal cannot unpack level %d.", i));
			break;
		}
	}

	image = Image::create_from_data(basisu_info.m_width, basisu_info.m_height, basisu_info.m_total_levels > 1, image_format, out_data);

	if (needs_ra_rg_swap) {
		// Swap uncompressed RA-as-RG texture's color channels.
		image->convert_ra_rgba8_to_rg();
	}

	return image;
}

Ref<Image> basis_universal_unpacker(const Vector<uint8_t> &p_buffer) {
	return basis_universal_unpacker_ptr(p_buffer.ptr(), p_buffer.size());
}
