partial class ExportedFields
{
#pragma warning disable CS0109 // Disable warning about redundant 'new' keyword
#if TOOLS
    /// <summary>
    /// Get the default values for all properties declared in this class.
    /// This method is used by Nebula to determine the value that will be
    /// used by the inspector when resetting properties.
    /// Do not call this method.
    /// </summary>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    internal new static global::System.Collections.Generic.Dictionary<global::Nebula.StringName, global::Nebula.Variant> GetNebulaPropertyDefaultValues()
    {
        var values = new global::System.Collections.Generic.Dictionary<global::Nebula.StringName, global::Nebula.Variant>(60);
        bool ___fieldBoolean_default_value = true;
        values.Add(PropertyName.@_fieldBoolean, global::Nebula.Variant.From<bool>(___fieldBoolean_default_value));
        char ___fieldChar_default_value = 'f';
        values.Add(PropertyName.@_fieldChar, global::Nebula.Variant.From<char>(___fieldChar_default_value));
        sbyte ___fieldSByte_default_value = 10;
        values.Add(PropertyName.@_fieldSByte, global::Nebula.Variant.From<sbyte>(___fieldSByte_default_value));
        short ___fieldInt16_default_value = 10;
        values.Add(PropertyName.@_fieldInt16, global::Nebula.Variant.From<short>(___fieldInt16_default_value));
        int ___fieldInt32_default_value = 10;
        values.Add(PropertyName.@_fieldInt32, global::Nebula.Variant.From<int>(___fieldInt32_default_value));
        long ___fieldInt64_default_value = 10;
        values.Add(PropertyName.@_fieldInt64, global::Nebula.Variant.From<long>(___fieldInt64_default_value));
        byte ___fieldByte_default_value = 10;
        values.Add(PropertyName.@_fieldByte, global::Nebula.Variant.From<byte>(___fieldByte_default_value));
        ushort ___fieldUInt16_default_value = 10;
        values.Add(PropertyName.@_fieldUInt16, global::Nebula.Variant.From<ushort>(___fieldUInt16_default_value));
        uint ___fieldUInt32_default_value = 10;
        values.Add(PropertyName.@_fieldUInt32, global::Nebula.Variant.From<uint>(___fieldUInt32_default_value));
        ulong ___fieldUInt64_default_value = 10;
        values.Add(PropertyName.@_fieldUInt64, global::Nebula.Variant.From<ulong>(___fieldUInt64_default_value));
        float ___fieldSingle_default_value = 10;
        values.Add(PropertyName.@_fieldSingle, global::Nebula.Variant.From<float>(___fieldSingle_default_value));
        double ___fieldDouble_default_value = 10;
        values.Add(PropertyName.@_fieldDouble, global::Nebula.Variant.From<double>(___fieldDouble_default_value));
        string ___fieldString_default_value = "foo";
        values.Add(PropertyName.@_fieldString, global::Nebula.Variant.From<string>(___fieldString_default_value));
        global::Nebula.Vector2 ___fieldVector2_default_value = new(10f, 10f);
        values.Add(PropertyName.@_fieldVector2, global::Nebula.Variant.From<global::Nebula.Vector2>(___fieldVector2_default_value));
        global::Nebula.Vector2I ___fieldVector2I_default_value = global::Nebula.Vector2I.Up;
        values.Add(PropertyName.@_fieldVector2I, global::Nebula.Variant.From<global::Nebula.Vector2I>(___fieldVector2I_default_value));
        global::Nebula.Rect2 ___fieldRect2_default_value = new(new global::Nebula.Vector2(10f, 10f), new global::Nebula.Vector2(10f, 10f));
        values.Add(PropertyName.@_fieldRect2, global::Nebula.Variant.From<global::Nebula.Rect2>(___fieldRect2_default_value));
        global::Nebula.Rect2I ___fieldRect2I_default_value = new(new global::Nebula.Vector2I(10, 10), new global::Nebula.Vector2I(10, 10));
        values.Add(PropertyName.@_fieldRect2I, global::Nebula.Variant.From<global::Nebula.Rect2I>(___fieldRect2I_default_value));
        global::Nebula.Transform2D ___fieldTransform2D_default_value = global::Nebula.Transform2D.Identity;
        values.Add(PropertyName.@_fieldTransform2D, global::Nebula.Variant.From<global::Nebula.Transform2D>(___fieldTransform2D_default_value));
        global::Nebula.Vector3 ___fieldVector3_default_value = new(10f, 10f, 10f);
        values.Add(PropertyName.@_fieldVector3, global::Nebula.Variant.From<global::Nebula.Vector3>(___fieldVector3_default_value));
        global::Nebula.Vector3I ___fieldVector3I_default_value = global::Nebula.Vector3I.Back;
        values.Add(PropertyName.@_fieldVector3I, global::Nebula.Variant.From<global::Nebula.Vector3I>(___fieldVector3I_default_value));
        global::Nebula.Basis ___fieldBasis_default_value = new global::Nebula.Basis(global::Nebula.Quaternion.Identity);
        values.Add(PropertyName.@_fieldBasis, global::Nebula.Variant.From<global::Nebula.Basis>(___fieldBasis_default_value));
        global::Nebula.Quaternion ___fieldQuaternion_default_value = new global::Nebula.Quaternion(global::Nebula.Basis.Identity);
        values.Add(PropertyName.@_fieldQuaternion, global::Nebula.Variant.From<global::Nebula.Quaternion>(___fieldQuaternion_default_value));
        global::Nebula.Transform3D ___fieldTransform3D_default_value = global::Nebula.Transform3D.Identity;
        values.Add(PropertyName.@_fieldTransform3D, global::Nebula.Variant.From<global::Nebula.Transform3D>(___fieldTransform3D_default_value));
        global::Nebula.Vector4 ___fieldVector4_default_value = new(10f, 10f, 10f, 10f);
        values.Add(PropertyName.@_fieldVector4, global::Nebula.Variant.From<global::Nebula.Vector4>(___fieldVector4_default_value));
        global::Nebula.Vector4I ___fieldVector4I_default_value = global::Nebula.Vector4I.One;
        values.Add(PropertyName.@_fieldVector4I, global::Nebula.Variant.From<global::Nebula.Vector4I>(___fieldVector4I_default_value));
        global::Nebula.Projection ___fieldProjection_default_value = global::Nebula.Projection.Identity;
        values.Add(PropertyName.@_fieldProjection, global::Nebula.Variant.From<global::Nebula.Projection>(___fieldProjection_default_value));
        global::Nebula.Aabb ___fieldAabb_default_value = new global::Nebula.Aabb(10f, 10f, 10f, new global::Nebula.Vector3(1f, 1f, 1f));
        values.Add(PropertyName.@_fieldAabb, global::Nebula.Variant.From<global::Nebula.Aabb>(___fieldAabb_default_value));
        global::Nebula.Color ___fieldColor_default_value = global::Nebula.Colors.Aquamarine;
        values.Add(PropertyName.@_fieldColor, global::Nebula.Variant.From<global::Nebula.Color>(___fieldColor_default_value));
        global::Nebula.Plane ___fieldPlane_default_value = global::Nebula.Plane.PlaneXZ;
        values.Add(PropertyName.@_fieldPlane, global::Nebula.Variant.From<global::Nebula.Plane>(___fieldPlane_default_value));
        global::Nebula.Callable ___fieldCallable_default_value = new global::Nebula.Callable(global::Nebula.Engine.GetMainLoop(), "_process");
        values.Add(PropertyName.@_fieldCallable, global::Nebula.Variant.From<global::Nebula.Callable>(___fieldCallable_default_value));
        global::Nebula.Signal ___fieldSignal_default_value = new global::Nebula.Signal(global::Nebula.Engine.GetMainLoop(), "property_list_changed");
        values.Add(PropertyName.@_fieldSignal, global::Nebula.Variant.From<global::Nebula.Signal>(___fieldSignal_default_value));
        global::ExportedFields.MyEnum ___fieldEnum_default_value = global::ExportedFields.MyEnum.C;
        values.Add(PropertyName.@_fieldEnum, global::Nebula.Variant.From<global::ExportedFields.MyEnum>(___fieldEnum_default_value));
        global::ExportedFields.MyFlagsEnum ___fieldFlagsEnum_default_value = global::ExportedFields.MyFlagsEnum.C;
        values.Add(PropertyName.@_fieldFlagsEnum, global::Nebula.Variant.From<global::ExportedFields.MyFlagsEnum>(___fieldFlagsEnum_default_value));
        byte[] ___fieldByteArray_default_value = { 0, 1, 2, 3, 4, 5, 6  };
        values.Add(PropertyName.@_fieldByteArray, global::Nebula.Variant.From<byte[]>(___fieldByteArray_default_value));
        int[] ___fieldInt32Array_default_value = { 0, 1, 2, 3, 4, 5, 6  };
        values.Add(PropertyName.@_fieldInt32Array, global::Nebula.Variant.From<int[]>(___fieldInt32Array_default_value));
        long[] ___fieldInt64Array_default_value = { 0, 1, 2, 3, 4, 5, 6  };
        values.Add(PropertyName.@_fieldInt64Array, global::Nebula.Variant.From<long[]>(___fieldInt64Array_default_value));
        float[] ___fieldSingleArray_default_value = { 0f, 1f, 2f, 3f, 4f, 5f, 6f  };
        values.Add(PropertyName.@_fieldSingleArray, global::Nebula.Variant.From<float[]>(___fieldSingleArray_default_value));
        double[] ___fieldDoubleArray_default_value = { 0d, 1d, 2d, 3d, 4d, 5d, 6d  };
        values.Add(PropertyName.@_fieldDoubleArray, global::Nebula.Variant.From<double[]>(___fieldDoubleArray_default_value));
        string[] ___fieldStringArray_default_value = { "foo", "bar"  };
        values.Add(PropertyName.@_fieldStringArray, global::Nebula.Variant.From<string[]>(___fieldStringArray_default_value));
        string[] ___fieldStringArrayEnum_default_value = { "foo", "bar"  };
        values.Add(PropertyName.@_fieldStringArrayEnum, global::Nebula.Variant.From<string[]>(___fieldStringArrayEnum_default_value));
        global::Nebula.Vector2[] ___fieldVector2Array_default_value = { global::Nebula.Vector2.Up, global::Nebula.Vector2.Down, global::Nebula.Vector2.Left, global::Nebula.Vector2.Right   };
        values.Add(PropertyName.@_fieldVector2Array, global::Nebula.Variant.From<global::Nebula.Vector2[]>(___fieldVector2Array_default_value));
        global::Nebula.Vector3[] ___fieldVector3Array_default_value = { global::Nebula.Vector3.Up, global::Nebula.Vector3.Down, global::Nebula.Vector3.Left, global::Nebula.Vector3.Right   };
        values.Add(PropertyName.@_fieldVector3Array, global::Nebula.Variant.From<global::Nebula.Vector3[]>(___fieldVector3Array_default_value));
        global::Nebula.Color[] ___fieldColorArray_default_value = { global::Nebula.Colors.Aqua, global::Nebula.Colors.Aquamarine, global::Nebula.Colors.Azure, global::Nebula.Colors.Beige   };
        values.Add(PropertyName.@_fieldColorArray, global::Nebula.Variant.From<global::Nebula.Color[]>(___fieldColorArray_default_value));
        global::Nebula.NebulaObject[] ___fieldNebulaObjectOrDerivedArray_default_value = { null  };
        values.Add(PropertyName.@_fieldNebulaObjectOrDerivedArray, global::Nebula.Variant.CreateFrom(___fieldNebulaObjectOrDerivedArray_default_value));
        global::Nebula.StringName[] ___fieldStringNameArray_default_value = { "foo", "bar"  };
        values.Add(PropertyName.@_fieldStringNameArray, global::Nebula.Variant.From<global::Nebula.StringName[]>(___fieldStringNameArray_default_value));
        global::Nebula.NodePath[] ___fieldNodePathArray_default_value = { "foo", "bar"  };
        values.Add(PropertyName.@_fieldNodePathArray, global::Nebula.Variant.From<global::Nebula.NodePath[]>(___fieldNodePathArray_default_value));
        global::Nebula.Rid[] ___fieldRidArray_default_value = { default, default, default  };
        values.Add(PropertyName.@_fieldRidArray, global::Nebula.Variant.From<global::Nebula.Rid[]>(___fieldRidArray_default_value));
        int[] ___fieldEmptyInt32Array_default_value = global::System.Array.Empty<int>();
        values.Add(PropertyName.@_fieldEmptyInt32Array, global::Nebula.Variant.From<int[]>(___fieldEmptyInt32Array_default_value));
        int[] ___fieldArrayFromList_default_value = new global::System.Collections.Generic.List<int>(global::System.Array.Empty<int>()).ToArray();
        values.Add(PropertyName.@_fieldArrayFromList, global::Nebula.Variant.From<int[]>(___fieldArrayFromList_default_value));
        global::Nebula.Variant ___fieldVariant_default_value = "foo";
        values.Add(PropertyName.@_fieldVariant, global::Nebula.Variant.From<global::Nebula.Variant>(___fieldVariant_default_value));
        global::Nebula.NebulaObject ___fieldNebulaObjectOrDerived_default_value = default;
        values.Add(PropertyName.@_fieldNebulaObjectOrDerived, global::Nebula.Variant.From<global::Nebula.NebulaObject>(___fieldNebulaObjectOrDerived_default_value));
        global::Nebula.Texture ___fieldNebulaResourceTexture_default_value = default;
        values.Add(PropertyName.@_fieldNebulaResourceTexture, global::Nebula.Variant.From<global::Nebula.Texture>(___fieldNebulaResourceTexture_default_value));
        global::Nebula.StringName ___fieldStringName_default_value = new global::Nebula.StringName("foo");
        values.Add(PropertyName.@_fieldStringName, global::Nebula.Variant.From<global::Nebula.StringName>(___fieldStringName_default_value));
        global::Nebula.NodePath ___fieldNodePath_default_value = new global::Nebula.NodePath("foo");
        values.Add(PropertyName.@_fieldNodePath, global::Nebula.Variant.From<global::Nebula.NodePath>(___fieldNodePath_default_value));
        global::Nebula.Rid ___fieldRid_default_value = default;
        values.Add(PropertyName.@_fieldRid, global::Nebula.Variant.From<global::Nebula.Rid>(___fieldRid_default_value));
        global::Nebula.Collections.Dictionary ___fieldNebulaDictionary_default_value = new()  { { "foo", 10  }, { global::Nebula.Vector2.Up, global::Nebula.Colors.Chocolate   }  };
        values.Add(PropertyName.@_fieldNebulaDictionary, global::Nebula.Variant.From<global::Nebula.Collections.Dictionary>(___fieldNebulaDictionary_default_value));
        global::Nebula.Collections.Array ___fieldNebulaArray_default_value = new()  { "foo", 10, global::Nebula.Vector2.Up, global::Nebula.Colors.Chocolate   };
        values.Add(PropertyName.@_fieldNebulaArray, global::Nebula.Variant.From<global::Nebula.Collections.Array>(___fieldNebulaArray_default_value));
        global::Nebula.Collections.Dictionary<string, bool> ___fieldNebulaGenericDictionary_default_value = new()  { { "foo", true  }, { "bar", false  }  };
        values.Add(PropertyName.@_fieldNebulaGenericDictionary, global::Nebula.Variant.CreateFrom(___fieldNebulaGenericDictionary_default_value));
        global::Nebula.Collections.Array<int> ___fieldNebulaGenericArray_default_value = new()  { 0, 1, 2, 3, 4, 5, 6  };
        values.Add(PropertyName.@_fieldNebulaGenericArray, global::Nebula.Variant.CreateFrom(___fieldNebulaGenericArray_default_value));
        long[] ___fieldEmptyInt64Array_default_value = global::System.Array.Empty<long>();
        values.Add(PropertyName.@_fieldEmptyInt64Array, global::Nebula.Variant.From<long[]>(___fieldEmptyInt64Array_default_value));
        return values;
    }
#endif // TOOLS
#pragma warning restore CS0109
}
