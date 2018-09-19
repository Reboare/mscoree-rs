// mscoree_wrapper.h - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

#import "mscorlib.tlb"  \
    high_property_prefixes("_get","_put","_putref")		\
    rename("ReportEvent", "InteropServices_ReportEvent") \
	rename("or", "ORR")

#include <comdef.h>

#include <MSCorEE.h>
#include <comip.h>

extern "C" {
#define RUST_HOST_CONTROL_GUID_STR "1E20D486-67C7-4CD6-B56B-41D2297D5B2F"
DEFINE_GUID(RUST_HOST_CONTROL_GUID,
	0x1e20d486, 0x67c7, 0x4cd6, 0xb5, 0x6b, 0x41, 0xd2, 0x29, 0x7d, 0x5b, 0x2f);

/* Implementation of IHostControl - 
 * this must be set via ICLRRuntimeHost.SetHostControl() 
 * before calling ICLRRuntimeHost.Start()
 * 
 * Let's me grab the AppDomainManager pointer.
 */
class __declspec(uuid(RUST_HOST_CONTROL_GUID_STR)) RustHostControl : public IHostControl {
    public:
        //required implementations
        virtual HRESULT __stdcall QueryInterface(const IID &iid, void **ppv);
        virtual ULONG __stdcall AddRef();
        virtual ULONG __stdcall Release();
        virtual HRESULT __stdcall GetHostManager(REFIID id, void **ppHostManager);
        virtual HRESULT __stdcall SetAppDomainManager(DWORD dwAppDomainID, IUnknown* pUnkAppDomainManager);
          //Our own methods
        virtual HRESULT __stdcall GetAppDomainManager(mscorlib::_AppDomainManager** ppUnkAppDomainManager);
        
        RustHostControl();
        virtual ~RustHostControl();
    private: 
        long refCount_m;
        mscorlib::_AppDomainManager* defaultDomainManager_m;
};


    RustHostControl* RustHostControl_new();
}

// class RustHostControl : public IHostControl {
// public:
// 	HRESULT __stdcall QueryInterface(const IID &iid, void **ppv);
// 	ULONG __stdcall AddRef();
// 	ULONG __stdcall Release();
// 	HRESULT __stdcall GetHostManager(REFIID id, void **ppHostManager);
// 	HRESULT __stdcall SetAppDomainManager(DWORD dwAppDomainID, IUnknown* pUnkAppDomainManager);
// 	RustHostControl();
// 	virtual ~RustHostControl();
// private:
// 	long refCount_m;
// };