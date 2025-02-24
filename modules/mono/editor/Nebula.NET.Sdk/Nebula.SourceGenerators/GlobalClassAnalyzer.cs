using System.Collections.Immutable;
using System.Linq;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.Diagnostics;

namespace Nebula.SourceGenerators
{
    [DiagnosticAnalyzer(LanguageNames.CSharp)]
    public sealed class GlobalClassAnalyzer : DiagnosticAnalyzer
    {
        public override ImmutableArray<DiagnosticDescriptor> SupportedDiagnostics
            => ImmutableArray.Create(
                Common.GlobalClassMustDeriveFromNebulaObjectRule,
                Common.GlobalClassMustNotBeGenericRule);

        public override void Initialize(AnalysisContext context)
        {
            context.ConfigureGeneratedCodeAnalysis(GeneratedCodeAnalysisFlags.None);
            context.EnableConcurrentExecution();
            context.RegisterSyntaxNodeAction(AnalyzeNode, SyntaxKind.ClassDeclaration);
        }

        private static void AnalyzeNode(SyntaxNodeAnalysisContext context)
        {
            // Return if not a type symbol or the type is not a global class.
            if (context.ContainingSymbol is not INamedTypeSymbol typeSymbol ||
                !typeSymbol.GetAttributes().Any(a => a.AttributeClass?.IsNebulaGlobalClassAttribute() ?? false))
                return;

            if (typeSymbol.IsGenericType)
            {
                context.ReportDiagnostic(Diagnostic.Create(
                    Common.GlobalClassMustNotBeGenericRule,
                    typeSymbol.Locations.FirstLocationWithSourceTreeOrDefault(),
                    typeSymbol.ToDisplayString()
                ));
            }

            if (!typeSymbol.InheritsFrom("NebulaSharp", NebulaClasses.NebulaObject))
            {
                context.ReportDiagnostic(Diagnostic.Create(
                    Common.GlobalClassMustDeriveFromNebulaObjectRule,
                    typeSymbol.Locations.FirstLocationWithSourceTreeOrDefault(),
                    typeSymbol.ToDisplayString()
                ));
            }
        }
    }
}
