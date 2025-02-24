partial class ExportedProperties
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
        var values = new global::System.Collections.Generic.Dictionary<global::Nebula.StringName, global::Nebula.Variant>(64);
        string __NotGenerateComplexLamdaProperty_default_value = default;
        values.Add(PropertyName.@NotGenerateComplexLamdaProperty, global::Nebula.Variant.From<string>(__NotGenerateComplexLamdaProperty_default_value));
        string __NotGenerateLamdaNoFieldProperty_default_value = default;
        values.Add(PropertyName.@NotGenerateLamdaNoFieldProperty, global::Nebula.Variant.From<string>(__NotGenerateLamdaNoFieldProperty_default_value));
        string __NotGenerateComplexReturnProperty_default_value = default;
        values.Add(PropertyName.@NotGenerateComplexReturnProperty, global::Nebula.Variant.From<string>(__NotGenerateComplexReturnProperty_default_value));
        string __NotGenerateReturnsProperty_default_value = default;
        values.Add(PropertyName.@NotGenerateReturnsProperty, global::Nebula.Variant.From<string>(__NotGenerateReturnsProperty_default_value));
        string __FullPropertyString_default_value = "FullPropertyString";
        values.Add(PropertyName.@FullPropertyString, global::Nebula.Variant.From<string>(__FullPropertyString_default_value));
        string __FullPropertyString_Complex_default_value = new string("FullPropertyString_Complex")   + global::System.Convert.ToInt32("1");
        values.Add(PropertyName.@FullPropertyString_Complex, global::Nebula.Variant.From<string>(__FullPropertyString_Complex_default_value));
        string __LamdaPropertyString_default_value = "LamdaPropertyString";
        values.Add(PropertyName.@LamdaPropertyString, global::Nebula.Variant.From<string>(__LamdaPropertyString_default_value));
        bool __PropertyBoolean_default_value = true;
        values.Add(PropertyName.@PropertyBoolean, global::Nebula.Variant.From<bool>(__PropertyBoolean_default_value));
        char __PropertyChar_default_value = 'f';
        values.Add(PropertyName.@PropertyChar, global::Nebula.Variant.From<char>(__PropertyChar_default_value));
        sbyte __PropertySByte_default_value = 10;
        values.Add(PropertyName.@PropertySByte, global::Nebula.Variant.From<sbyte>(__PropertySByte_default_value));
        short __PropertyInt16_default_value = 10;
        values.Add(PropertyName.@PropertyInt16, global::Nebula.Variant.From<short>(__PropertyInt16_default_value));
        int __PropertyInt32_default_value = 10;
        values.Add(PropertyName.@PropertyInt32, global::Nebula.Variant.From<int>(__PropertyInt32_default_value));
        long __PropertyInt64_default_value = 10;
        values.Add(PropertyName.@PropertyInt64, global::Nebula.Variant.From<long>(__PropertyInt64_default_value));
        byte __PropertyByte_default_value = 10;
        values.Add(PropertyName.@PropertyByte, global::Nebula.Variant.From<byte>(__PropertyByte_default_value));
        ushort __PropertyUInt16_default_value = 10;
        values.Add(PropertyName.@PropertyUInt16, global::Nebula.Variant.From<ushort>(__PropertyUInt16_default_value));
        uint __PropertyUInt32_default_value = 10;
        values.Add(PropertyName.@PropertyUInt32, global::Nebula.Variant.From<uint>(__PropertyUInt32_default_value));
        ulong __PropertyUInt64_default_value = 10;
        values.Add(PropertyName.@PropertyUInt64, global::Nebula.Variant.From<ulong>(__PropertyUInt64_default_value));
        float __PropertySingle_default_value = 10;
        values.Add(PropertyName.@PropertySingle, global::Nebula.Variant.From<float>(__PropertySingle_default_value));
        double __PropertyDouble_default_value = 10;
        values.Add(PropertyName.@PropertyDouble, global::Nebula.Variant.From<double>(__PropertyDouble_default_value));
        string __PropertyString_default_value = "foo";
        values.Add(PropertyName.@PropertyString, global::Nebula.Variant.From<string>(__PropertyString_default_value));
        global::Nebula.Vector2 __PropertyVector2_default_value = new(10f, 10f);
        values.Add(PropertyName.@PropertyVector2, global::Nebula.Variant.From<global::Nebula.Vector2>(__PropertyVector2_default_value));
        global::Nebula.Vector2I __PropertyVector2I_default_value = global::Nebula.Vector2I.Up;
        values.Add(PropertyName.@PropertyVector2I, global::Nebula.Variant.From<global::Nebula.Vector2I>(__PropertyVector2I_default_value));
        global::Nebula.Rect2 __PropertyRect2_default_value = new(new global::Nebula.Vector2(10f, 10f), new global::Nebula.Vector2(10f, 10f));
        values.Add(PropertyName.@PropertyRect2, global::Nebula.Variant.From<global::Nebula.Rect2>(__PropertyRect2_default_value));
        global::Nebula.Rect2I __PropertyRect2I_default_value = new(new global::Nebula.Vector2I(10, 10), new global::Nebula.Vector2I(10, 10));
        values.Add(PropertyName.@PropertyRect2I, global::Nebula.Variant.From<global::Nebula.Rect2I>(__PropertyRect2I_default_value));
        global::Nebula.Transform2D __PropertyTransform2D_default_value = global::Nebula.Transform2D.Identity;
        values.Add(PropertyName.@PropertyTransform2D, global::Nebula.Variant.From<global::Nebula.Transform2D>(__PropertyTransform2D_default_value));
        global::Nebula.Vector3 __PropertyVector3_default_value = new(10f, 10f, 10f);
        values.Add(PropertyName.@PropertyVector3, global::Nebula.Variant.From<global::Nebula.Vector3>(__PropertyVector3_default_value));
        global::Nebula.Vector3I __PropertyVector3I_default_value = global::Nebula.Vector3I.Back;
        values.Add(PropertyName.@PropertyVector3I, global::Nebula.Variant.From<global::Nebula.Vector3I>(__PropertyVector3I_default_value));
        global::Nebula.Basis __PropertyBasis_default_value = new global::Nebula.Basis(global::Nebula.Quaternion.Identity);
        values.Add(PropertyName.@PropertyBasis, global::Nebula.Variant.From<global::Nebula.Basis>(__PropertyBasis_default_value));
        global::Nebula.Quaternion __PropertyQuaternion_default_value = new global::Nebula.Quaternion(global::Nebula.Basis.Identity);
        values.Add(PropertyName.@PropertyQuaternion, global::Nebula.Variant.From<global::Nebula.Quaternion>(__PropertyQuaternion_default_value));
        global::Nebula.Transform3D __PropertyTransform3D_default_value = global::Nebula.Transform3D.Identity;
        values.Add(PropertyName.@PropertyTransform3D, global::Nebula.Variant.From<global::Nebula.Transform3D>(__PropertyTransform3D_default_value));
        global::Nebula.Vector4 __PropertyVector4_default_value = new(10f, 10f, 10f, 10f);
        values.Add(PropertyName.@PropertyVector4, global::Nebula.Variant.From<global::Nebula.Vector4>(__PropertyVector4_default_value));
        global::Nebula.Vector4I __PropertyVector4I_default_value = global::Nebula.Vector4I.One;
        values.Add(PropertyName.@PropertyVector4I, global::Nebula.Variant.From<global::Nebula.Vector4I>(__PropertyVector4I_default_value));
        global::Nebula.Projection __PropertyProjection_default_value = global::Nebula.Projection.Identity;
        values.Add(PropertyName.@PropertyProjection, global::Nebula.Variant.From<global::Nebula.Projection>(__PropertyProjection_default_value));
        global::Nebula.Aabb __PropertyAabb_default_value = new global::Nebula.Aabb(10f, 10f, 10f, new global::Nebula.Vector3(1f, 1f, 1f));
        values.Add(PropertyName.@PropertyAabb, global::Nebula.Variant.From<global::Nebula.Aabb>(__PropertyAabb_default_value));
        global::Nebula.Color __PropertyColor_default_value = global::Nebula.Colors.Aquamarine;
        values.Add(PropertyName.@PropertyColor, global::Nebula.Variant.From<global::Nebula.Color>(__PropertyColor_default_value));
        global::Nebula.Plane __PropertyPlane_default_value = global::Nebula.Plane.PlaneXZ;
        values.Add(PropertyName.@PropertyPlane, global::Nebula.Variant.From<global::Nebula.Plane>(__PropertyPlane_default_value));
        global::Nebula.Callable __PropertyCallable_default_value = new global::Nebula.Callable(global::Nebula.Engine.GetMainLoop(), "_process");
        values.Add(PropertyName.@PropertyCallable, global::Nebula.Variant.From<global::Nebula.Callable>(__PropertyCallable_default_value));
        global::Nebula.Signal __PropertySignal_default_value = new global::Nebula.Signal(global::Nebula.Engine.GetMainLoop(), "Propertylist_changed");
        values.Add(PropertyName.@PropertySignal, global::Nebula.Variant.From<global::Nebula.Signal>(__PropertySignal_default_value));
        global::ExportedProperties.MyEnum __PropertyEnum_default_value = global::ExportedProperties.MyEnum.C;
        values.Add(PropertyName.@PropertyEnum, global::Nebula.Variant.From<global::ExportedProperties.MyEnum>(__PropertyEnum_default_value));
        global::ExportedProperties.MyFlagsEnum __PropertyFlagsEnum_default_value = global::ExportedProperties.MyFlagsEnum.C;
        values.Add(PropertyName.@PropertyFlagsEnum, global::Nebula.Variant.From<global::ExportedProperties.MyFlagsEnum>(__PropertyFlagsEnum_default_value));
        byte[] __PropertyByteArray_default_value = { 0, 1, 2, 3, 4, 5, 6  };
        values.Add(PropertyName.@PropertyByteArray, global::Nebula.Variant.From<byte[]>(__PropertyByteArray_default_value));
        int[] __PropertyInt32Array_default_value = { 0, 1, 2, 3, 4, 5, 6  };
        values.Add(PropertyName.@PropertyInt32Array, global::Nebula.Variant.From<int[]>(__PropertyInt32Array_default_value));
        long[] __PropertyInt64Array_default_value = { 0, 1, 2, 3, 4, 5, 6  };
        values.Add(PropertyName.@PropertyInt64Array, global::Nebula.Variant.From<long[]>(__PropertyInt64Array_default_value));
        float[] __PropertySingleArray_default_value = { 0f, 1f, 2f, 3f, 4f, 5f, 6f  };
        values.Add(PropertyName.@PropertySingleArray, global::Nebula.Variant.From<float[]>(__PropertySingleArray_default_value));
        double[] __PropertyDoubleArray_default_value = { 0d, 1d, 2d, 3d, 4d, 5d, 6d  };
        values.Add(PropertyName.@PropertyDoubleArray, global::Nebula.Variant.From<double[]>(__PropertyDoubleArray_default_value));
        string[] __PropertyStringArray_default_value = { "foo", "bar"  };
        values.Add(PropertyName.@PropertyStringArray, global::Nebula.Variant.From<string[]>(__PropertyStringArray_default_value));
        string[] __PropertyStringArrayEnum_default_value = { "foo", "bar"  };
        values.Add(PropertyName.@PropertyStringArrayEnum, global::Nebula.Variant.From<string[]>(__PropertyStringArrayEnum_default_value));
        global::Nebula.Vector2[] __PropertyVector2Array_default_value = { global::Nebula.Vector2.Up, global::Nebula.Vector2.Down, global::Nebula.Vector2.Left, global::Nebula.Vector2.Right   };
        values.Add(PropertyName.@PropertyVector2Array, global::Nebula.Variant.From<global::Nebula.Vector2[]>(__PropertyVector2Array_default_value));
        global::Nebula.Vector3[] __PropertyVector3Array_default_value = { global::Nebula.Vector3.Up, global::Nebula.Vector3.Down, global::Nebula.Vector3.Left, global::Nebula.Vector3.Right   };
        values.Add(PropertyName.@PropertyVector3Array, global::Nebula.Variant.From<global::Nebula.Vector3[]>(__PropertyVector3Array_default_value));
        global::Nebula.Color[] __PropertyColorArray_default_value = { global::Nebula.Colors.Aqua, global::Nebula.Colors.Aquamarine, global::Nebula.Colors.Azure, global::Nebula.Colors.Beige   };
        values.Add(PropertyName.@PropertyColorArray, global::Nebula.Variant.From<global::Nebula.Color[]>(__PropertyColorArray_default_value));
        global::Nebula.NebulaObject[] __PropertyNebulaObjectOrDerivedArray_default_value = { null  };
        values.Add(PropertyName.@PropertyNebulaObjectOrDerivedArray, global::Nebula.Variant.CreateFrom(__PropertyNebulaObjectOrDerivedArray_default_value));
        global::Nebula.StringName[] __field_StringNameArray_default_value = { "foo", "bar"  };
        values.Add(PropertyName.@field_StringNameArray, global::Nebula.Variant.From<global::Nebula.StringName[]>(__field_StringNameArray_default_value));
        global::Nebula.NodePath[] __field_NodePathArray_default_value = { "foo", "bar"  };
        values.Add(PropertyName.@field_NodePathArray, global::Nebula.Variant.From<global::Nebula.NodePath[]>(__field_NodePathArray_default_value));
        global::Nebula.Rid[] __field_RidArray_default_value = { default, default, default  };
        values.Add(PropertyName.@field_RidArray, global::Nebula.Variant.From<global::Nebula.Rid[]>(__field_RidArray_default_value));
        global::Nebula.Variant __PropertyVariant_default_value = "foo";
        values.Add(PropertyName.@PropertyVariant, global::Nebula.Variant.From<global::Nebula.Variant>(__PropertyVariant_default_value));
        global::Nebula.NebulaObject __PropertyNebulaObjectOrDerived_default_value = default;
        values.Add(PropertyName.@PropertyNebulaObjectOrDerived, global::Nebula.Variant.From<global::Nebula.NebulaObject>(__PropertyNebulaObjectOrDerived_default_value));
        global::Nebula.Texture __PropertyNebulaResourceTexture_default_value = default;
        values.Add(PropertyName.@PropertyNebulaResourceTexture, global::Nebula.Variant.From<global::Nebula.Texture>(__PropertyNebulaResourceTexture_default_value));
        global::Nebula.StringName __PropertyStringName_default_value = new global::Nebula.StringName("foo");
        values.Add(PropertyName.@PropertyStringName, global::Nebula.Variant.From<global::Nebula.StringName>(__PropertyStringName_default_value));
        global::Nebula.NodePath __PropertyNodePath_default_value = new global::Nebula.NodePath("foo");
        values.Add(PropertyName.@PropertyNodePath, global::Nebula.Variant.From<global::Nebula.NodePath>(__PropertyNodePath_default_value));
        global::Nebula.Rid __PropertyRid_default_value = default;
        values.Add(PropertyName.@PropertyRid, global::Nebula.Variant.From<global::Nebula.Rid>(__PropertyRid_default_value));
        global::Nebula.Collections.Dictionary __PropertyNebulaDictionary_default_value = new()  { { "foo", 10  }, { global::Nebula.Vector2.Up, global::Nebula.Colors.Chocolate   }  };
        values.Add(PropertyName.@PropertyNebulaDictionary, global::Nebula.Variant.From<global::Nebula.Collections.Dictionary>(__PropertyNebulaDictionary_default_value));
        global::Nebula.Collections.Array __PropertyNebulaArray_default_value = new()  { "foo", 10, global::Nebula.Vector2.Up, global::Nebula.Colors.Chocolate   };
        values.Add(PropertyName.@PropertyNebulaArray, global::Nebula.Variant.From<global::Nebula.Collections.Array>(__PropertyNebulaArray_default_value));
        global::Nebula.Collections.Dictionary<string, bool> __PropertyNebulaGenericDictionary_default_value = new()  { { "foo", true  }, { "bar", false  }  };
        values.Add(PropertyName.@PropertyNebulaGenericDictionary, global::Nebula.Variant.CreateFrom(__PropertyNebulaGenericDictionary_default_value));
        global::Nebula.Collections.Array<int> __PropertyNebulaGenericArray_default_value = new()  { 0, 1, 2, 3, 4, 5, 6  };
        values.Add(PropertyName.@PropertyNebulaGenericArray, global::Nebula.Variant.CreateFrom(__PropertyNebulaGenericArray_default_value));
        return values;
    }
#endif // TOOLS
#pragma warning restore CS0109
}
