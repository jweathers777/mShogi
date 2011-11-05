////////////////////////////////////////////////////////////////////////////
// Name:         GameSetupDlg.hpp
// Description:  Dialog for setting up a game in mShogi
// Created:      08/12/2004 10:28:40 Eastern Standard Time
// Last Updated: $Date: 2004/08/28 02:50:49 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __GameSetupDlg_HPP__
#define __GameSetupDlg_HPP__

#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include "mShogi_wdr.h"

// WDR: class declarations

//----------------------------------------------------------------------------
// GameSetupDlg
//----------------------------------------------------------------------------

class GameSetupDlg : public wxDialog
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   GameSetupDlg( wxWindow* parent, const wxString &title,
                 wxString* items,  int nItems);

   // ======================================================================
   // Operations
   // ======================================================================

   // WDR: method declarations for GameSetupDlg
   wxRadioBox* GetRdbBplayer() { return (wxRadioBox*) FindWindow( ID_RDB_BPLAYER ); }
   wxRadioBox* GetRdbWplayer() { return (wxRadioBox*) FindWindow( ID_RDB_WPLAYER ); }
   wxListBox*  GetLstGame()    { return (wxListBox*) FindWindow( ID_LST_GAME ); }
   wxStaticBitmap* GetBmpFlipboard()  { return (wxStaticBitmap*) FindWindow( ID_BMP_FLIPBOARD ); }

private:
   // ======================================================================
   // Data
   // ======================================================================

    // WDR: member variable declarations for GameSetupDlg
    bool mBoardFlipped;
    
   // ======================================================================
   // Event Handlers
   // ======================================================================
   
   // WDR: handler declarations for GameSetupDlg
   void OnPressFlipButton( wxCommandEvent &event );

   DECLARE_EVENT_TABLE()
};

#endif

