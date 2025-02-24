using Nebula;
using System;
using NebulaTools.ProjectEditor;

namespace NebulaTools
{
    public static class CsProjOperations
    {
        public static string GenerateGameProject(string dir, string name)
        {
            try
            {
                return ProjectGenerator.GenAndSaveGameProject(dir, name);
            }
            catch (Exception e)
            {
                GD.PushError(e.ToString());
                return string.Empty;
            }
        }
    }
}
