using System;
using System.Globalization;
using System.IO;
using System.Text;
using Microsoft.Build.Construction;
using Microsoft.Build.Evaluation;
using NebulaTools.Shared;

namespace NebulaTools.ProjectEditor
{
    public static class ProjectGenerator
    {
        public static string NebulaSdkAttrValue => $"Nebula.NET.Sdk/{GeneratedNebulaNupkgsVersions.NebulaNETSdk}";

        public static ProjectRootElement GenGameProject(string name)
        {
            if (name.Length == 0)
                throw new ArgumentException("Project name is empty.", nameof(name));

            var root = ProjectRootElement.Create(NewProjectFileOptions.None);

            root.Sdk = NebulaSdkAttrValue;

            var mainGroup = root.AddPropertyGroup();
            mainGroup.AddProperty("TargetFramework", "net6.0");

            var net7 = mainGroup.AddProperty("TargetFramework", "net7.0");
            net7.Condition = " '$(NebulaTargetPlatform)' == 'android' ";

            var net8 = mainGroup.AddProperty("TargetFramework", "net8.0");
            net8.Condition = " '$(NebulaTargetPlatform)' == 'ios' ";

            mainGroup.AddProperty("EnableDynamicLoading", "true");

            string sanitizedName = IdentifierUtils.SanitizeQualifiedIdentifier(name, allowEmptyIdentifiers: true);

            // If the name is not a valid namespace, manually set RootNamespace to a sanitized one.
            if (sanitizedName != name)
                mainGroup.AddProperty("RootNamespace", sanitizedName);

            return root;
        }

        public static string GenAndSaveGameProject(string dir, string name)
        {
            if (name.Length == 0)
                throw new ArgumentException("Project name is empty.", nameof(name));

            string path = Path.Combine(dir, name + ".csproj");

            var root = GenGameProject(name);

            // Save (without BOM)
            root.Save(path, new UTF8Encoding(encoderShouldEmitUTF8Identifier: false));

            return Guid.NewGuid().ToString().ToUpperInvariant();
        }
    }
}
