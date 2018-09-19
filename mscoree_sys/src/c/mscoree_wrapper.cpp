#include "mscoree_wrapper.h"
#include <stdio.h>
#include <iostream>

extern "C" {
	RustHostControl* RustHostControl_new() {
		return new RustHostControl();
	}	
}

RustHostControl::RustHostControl() {
    refCount_m = 0;
}

RustHostControl::~RustHostControl() {

}

HRESULT RustHostControl::QueryInterface(const IID & iid, void ** ppv)
{
	if (!ppv) return E_POINTER;
	*ppv = this;
	AddRef();
	return S_OK;
}

ULONG RustHostControl::AddRef()
{
	return InterlockedIncrement(&refCount_m);
}

ULONG RustHostControl::Release()
{
	if (InterlockedDecrement(&refCount_m) == 0) {
		delete this;
		return 0;
	}
	return refCount_m;
}

HRESULT RustHostControl::GetHostManager(REFIID id, void ** ppHostManager)
{
	*ppHostManager = NULL;
	return E_NOINTERFACE;
}

HRESULT RustHostControl::SetAppDomainManager(DWORD dwAppDomainID, IUnknown * pUnkAppDomainManager)
{
	std::cout << "Inside SetAppDomainManager\n";
	HRESULT hr = S_OK;
	hr = pUnkAppDomainManager->QueryInterface(__uuidof(mscorlib::_AppDomainManager), (PVOID*)&defaultDomainManager_m);
	return hr;
}

HRESULT __stdcall RustHostControl::GetAppDomainManager(mscorlib::_AppDomainManager** ppUnkAppDomainManager) {
	*ppUnkAppDomainManager = defaultDomainManager_m;
	return S_OK;
}