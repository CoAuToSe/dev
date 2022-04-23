/*------------------------------------------------------------------------------ 
Copyright (c) Microsoft Corporation.  All rights reserved.

@doc 

@module IVsSccAddWebProjectFromSourceControl.idl - The IDL File describing the "Add Web Project From Source Control" interface |
IDL source for interface allowing adding a web project from source control and.

@owner Microsoft Corporation, Visual Studio Core Team
------------------------------------------------------------------------------*/

#if !defined(CTC_INVOKED) && !defined(RGS_INVOKED)

cpp_quote("#pragma once")

//------------------------------------------------------------------------------
// Imports
#ifndef INTEROPLIB

import "oaidl.idl";

#endif

//------------------------------------------------------------------------------
//  How this interface should be used:
//------------------------------------------------------------------------------
//
//  VB/C#/etc project will query this interface from SccMangager and they will be guaranteed to receive a non-NULL pointer
//  because the interface is implemented by the Scci integration package.
//
//  To determine whether the active scc provider supports indeed web functionality, the project system should call
//  IsAddWebProjectSupported(). If the scenario is supported, the controls on the AddWeb...FromSourceControl dialog will be enabled.
//
//  When the user clicks the Browse button in such dialog, the project system will call BrowseForServerLocation().
//  The returned values are: suggested local path, displayable server location, some opaque strings identifying the provider and server location
//  Examples of strings that can be returned by BrowseForServerLocation():
//
//	VisualSourceSafe 7.0:
//			'WebProject1 from FriendlyDatabaseName'
//			'C:\Web Projects\WebProject1'
//			'msss://SomeServer/SourceSafe70Share:/~files:Ws="SSWorkspace"/Web Projects/WebProject1'
//			''
//			'{53544C4D-B03D-4209-A7D0-D9DD13A4019B}:VAPI:VSS70'
//
//	VisualSourceSafe 6.0 using MSSCCI:
//			'$/Web Projects/WebProject1 from \\SomeServer\SourceSafe70Share'
//			'C:\Web Projects\WebProject1'
//			'$/We