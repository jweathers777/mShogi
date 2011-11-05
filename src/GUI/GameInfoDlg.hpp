////////////////////////////////////////////////////////////////////////////
// Name:         GameInfoDlg.hpp
// Description:  Dialog for setting up game information in mShogi
// Created:      10/06/2004 07:30:50 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __GameInfoDlg_HPP__
#define __GameInfoDlg_HPP__

#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include "mShogi_wdr.h"

class GameInfoDlg : public wxDialog
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   GameInfoDlg( wxWindow* parent, const wxString& title);

   // ======================================================================
   // Operations
   // ======================================================================

    wxTextCtrl* GetTxtTitle()  { return (wxTextCtrl*) FindWindow( ID_TXT_TITLE ); }
    
    wxTextCtrl* GetTxtBlack()  { return (wxTextCtrl*) FindWindow( ID_TXT_BLACK ); }
    wxTextCtrl* GetTxtBtitle()  { return (wxTextCtrl*) FindWindow( ID_TXT_BTITLE ); }
    wxTextCtrl* GetTxtBrating()  { return (wxTextCtrl*) FindWindow( ID_TXT_BRATING ); }
    wxTextCtrl* GetTxtBcountry()  { return (wxTextCtrl*) FindWindow( ID_TXT_BCOUNTRY ); }
    
    wxTextCtrl* GetTxtWhite()  { return (wxTextCtrl*) FindWindow( ID_TXT_WHITE ); }
    wxTextCtrl* GetTxtWtitle()  { return (wxTextCtrl*) FindWindow( ID_TXT_WTITLE ); }
    wxTextCtrl* GetTxtWrating()  { return (wxTextCtrl*) FindWindow( ID_TXT_WRATING ); }
    wxTextCtrl* GetTxtWcountry()  { return (wxTextCtrl*) FindWindow( ID_TXT_WCOUNTRY ); }
    
    wxTextCtrl* GetTxtEvent()  { return (wxTextCtrl*) FindWindow( ID_TXT_EVENT ); }
    wxTextCtrl* GetTxtRound()  { return (wxTextCtrl*) FindWindow( ID_TXT_ROUND ); }
    wxTextCtrl* GetTxtSite()  { return (wxTextCtrl*) FindWindow( ID_TXT_SITE ); }
    wxTextCtrl* GetTxtDate()  { return (wxTextCtrl*) FindWindow( ID_TXT_DATE ); }
    wxTextCtrl* GetTxtAnnotator()  { return (wxTextCtrl*) FindWindow( ID_TXT_ANNOTATOR ); }
    wxTextCtrl* GetTxtArating()  { return (wxTextCtrl*) FindWindow( ID_TXT_ARATING ); }
    wxTextCtrl* GetTxtRemarks()  { return (wxTextCtrl*) FindWindow( ID_TXT_REMARKS ); }
    
    wxRadioBox* GetRdbResults()  { return (wxRadioBox*) FindWindow( ID_RDB_RESULTS ); }
   
   // ======================================================================
   // Event Handlers
   // ======================================================================


   //DECLARE_EVENT_TABLE()
};

#endif

