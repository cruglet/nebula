using System;
using System.Runtime.CompilerServices;
using Nebula;
using Nebula.NativeInterop;

namespace NebulaTools.Internals
{
    public class EditorProgress : IDisposable
    {
        public string Task { get; }

        public EditorProgress(string task, string label, int amount, bool canCancel = false)
        {
            Task = task;
            using nebula_string taskIn = Marshaling.ConvertStringToNative(task);
            using nebula_string labelIn = Marshaling.ConvertStringToNative(label);
            Internal.nebula_icall_EditorProgress_Create(taskIn, labelIn, amount, canCancel);
        }

        ~EditorProgress()
        {
            // Should never rely on the GC to dispose EditorProgress.
            // It should be disposed immediately when the task finishes.
            GD.PushError("EditorProgress disposed by the Garbage Collector");
            Dispose();
        }

        public void Dispose()
        {
            using nebula_string taskIn = Marshaling.ConvertStringToNative(Task);
            Internal.nebula_icall_EditorProgress_Dispose(taskIn);
            GC.SuppressFinalize(this);
        }

        public void Step(string state, int step = -1, bool forceRefresh = true)
        {
            using nebula_string taskIn = Marshaling.ConvertStringToNative(Task);
            using nebula_string stateIn = Marshaling.ConvertStringToNative(state);
            Internal.nebula_icall_EditorProgress_Step(taskIn, stateIn, step, forceRefresh);
        }

        public bool TryStep(string state, int step = -1, bool forceRefresh = true)
        {
            using nebula_string taskIn = Marshaling.ConvertStringToNative(Task);
            using nebula_string stateIn = Marshaling.ConvertStringToNative(state);
            return Internal.nebula_icall_EditorProgress_Step(taskIn, stateIn, step, forceRefresh);
        }
    }
}
