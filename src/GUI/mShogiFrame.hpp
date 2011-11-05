////////////////////////////////////////////////////////////////////////////
// Name:         mShogiFrame.hpp
// Description:  Frame class for mShogi
// Created:      08/10/2004 04:16:59 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 02:50:49 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __mShogiFrame_HPP__
#define __mShogiFrame_HPP__

// Forward declarations
class GameSetupDlg;
class GameInfoDlg;
class Engine;
class BoardCanvas;

class mShogiFrame : public wxFrame
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   mShogiFrame( wxWindow *parent, wxWindowID id, const wxString &title,
                const wxPoint& pos = wxDefaultPosition,
                const wxSize& size = wxDefaultSize,
                long style = wxDEFAULT_FRAME_STYLE );
      
   ~mShogiFrame();

private:
   // ======================================================================
   // Initialization
   // ======================================================================
   void CreateTheMenuBar();
   void CreateTheToolBar();
   
   // ======================================================================
   // Widgets
   // ======================================================================   
   BoardCanvas*  mpBoardCanvas;    // Canvas on which the board is displayed
   GameSetupDlg* mpGameSetupDlg;   // Game Setup Dialog box
   GameInfoDlg*  mpGameInfoDlg;    // Game information dialog box
   
   // ======================================================================
   // Data
   // ======================================================================   
   Engine*       mpEngine;         // Pointer to game engine
   
   int mNotationStyle;

   // ======================================================================
   // Event Handlers
   // ======================================================================
   void OnLoad( wxCommandEvent &event );
   void OnSave( wxCommandEvent &event );
   void OnQuit( wxCommandEvent &event );
   void OnCloseWindow( wxCloseEvent &event );
   
   void OnNew( wxCommandEvent &event );
   void OnGameInformation( wxCommandEvent &event);
   
   void OnTakeBack( wxCommandEvent &event );
   void OnForce( wxCommandEvent &event );
   
   void OnAbout( wxCommandEvent &event );
      
   DECLARE_EVENT_TABLE()
};

#endif
