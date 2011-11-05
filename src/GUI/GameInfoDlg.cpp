////////////////////////////////////////////////////////////////////////////
// Name:         GameInfoDlg.cpp
// Description:  Dialog for setting up game information in mShogi
// Created:      10/06/2004 07:30:51 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// =========================================================================
// Header file includes
// =========================================================================

// For compilers that support precompilation, includes "wx/wx.h".
#include "wx/wxprec.h"

#ifdef __BORLANDC__
    #pragma hdrstop
#endif

#ifdef __WXMSW__
#define FindWindowA FindWindow
#define GetCharWidthA GetCharWidth
#endif

// Local header files
#include "common.hpp"
#include "GameInfoDlg.hpp"

// WDR: class implementations

//----------------------------------------------------------------------------
// GameInfoDlg
//----------------------------------------------------------------------------

// ============================================================================
// Event Tables
// ============================================================================

// WDR: event table for GameSetupDlg

//--------------------------------------------------------------------------
//       Class:  GameInfoDlg
//      Method:  GameInfoDlg
// Description:  Construct an instance of a GameSetupDlg object
//--------------------------------------------------------------------------
GameInfoDlg::GameInfoDlg( wxWindow *parent, const wxString &title)
: wxDialog( parent, -1, title, wxDefaultPosition, wxDefaultSize, 
            wxDEFAULT_DIALOG_STYLE )
{
   // WDR: dialog function InitGameInfo for GameInfoDlg
}

// WDR: handler implementations for GameInfoDlg

