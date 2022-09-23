using System;
using System.Runtime.InteropServices;

public unsafe class RustApp
{



    [DllImport(@"lib/c_sharp_ffi.dll")]
    private static extern void* cg_new();

    [DllImport(@"lib/c_sharp_ffi.dll")]
    private static extern void cg_tick(ref void* state);

    [DllImport(@"lib/c_sharp_ffi.dll")]
    public static extern IntPtr cg_resource_fetch_string_utf8(ref void* state, object id);


    [DllImport(@"lib/c_sharp_ffi.dll")]
    public static extern object cb_get_string_id();




    void* _state = null;



    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    delegate void test_add(Int32 a, Int32 b);

    public RustApp()
    {

        _state = cg_new();
        for (int i = 0; i < 10; i++)
        {
            cg_tick(ref _state);
        }

        var x = cg_resource_fetch_string_utf8(ref _state, cb_get_string_id());
        Console.WriteLine($"{x}");
        if (x != null)
        {
            var text = Marshal.PtrToStringUTF8(x);
            Console.WriteLine($"{text}");
        }


    }
}
