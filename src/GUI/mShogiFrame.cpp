////////////////////////////////////////////////////////////////////////////
// Name:         mShogiFrame.cpp
// Description:  Frame class for mShogi
// Created:      08/10/2004 04:23:41 Eastern Daylight Time
// Last Updated: $Date: 2004/09/30 02:02:58 $
// Revision:     $Revision: 1.2 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// ===========================================================================
// Header file includes
// ===========================================================================

// wxWidget header files
#include "wx/wx.h"

// wxDesigner generated header file
#include "mShogi_wdr.h"

#define wxUSE_GENERIC_DRAGIMAGE 1

#if wxUSE_GENERIC_DRAGIMAGE
#include "wx/generic/dragimgg.h"
#define wxDragImage wxGenericDragImage
#else
#include "wx/dragimag.h"
#endif

#ifdef __WXMSW__
#define FindWindowA FindWindow
#endif

// STL header files
#include <vector>
#include <string>

// Local header files
#include "common.hpp"
#include "mShogiFrame.hpp"
#include "mShogiApp.hpp"
#include "Engine.hpp"
#include "BoardCanvas.hpp"
#include "GameSetupDlg.hpp"
#include "GameInfoDlg.hpp"

using std::vector;
using std::string;

// ----------------------------------------------------------------------------
// resources
// ----------------------------------------------------------------------------
// the application icon
#if defined(__WXGTK__) || defined(__WXMOTIF__)
   #include "mShogiIcon.xpm"
#endif

// ============================================================================
// Event Tables
// ============================================================================
BEGIN_EVENT_TABLE(mShogiFrame,wxFrame)
   EVT_MENU(ID_LOAD,  mShogiFrame::OnLoad)
   EVT_MENU(ID_SAVE,  mShogiFrame::OnSave)
   EVT_MENU(ID_EXIT,  mShogiFrame::OnQuit)
   EVT_MENU(ID_NEW,   mShogiFrame::OnNew)
   EVT_MENU(ID_GAME_INFORMATION,   mShogiFrame::OnGameInformation)
   EVT_MENU(ID_TAKEBACK, mShogiFrame::OnTakeBack)
   EVT_MENU(ID_FORCE,   mShogiFrame::OnForce)
   EVT_MENU(ID_ABOUT, mShogiFrame::OnAbout)
   EVT_CLOSE(mShogiFrame::OnCloseWindow)
END_EVENT_TABLE()

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  mShogiFrame
// Description:  Construct an instance of a mShogiFrame object
//--------------------------------------------------------------------------
mShogiFrame::mShogiFrame( wxWindow *parent, wxWindowID id, 
                          const wxString &title, const wxPoint &position, 
                          const wxSize& size, long style ) 
: wxFrame( parent, id, title, position, size, style )
{
   // Construct UI elements
   CreateTheMenuBar();
   CreateTheToolBar();
   
   CreateStatusBar(3);
   SetStatusText( wxT("Ready"), 0 );
   SetStatusText( wxT(""), 1 );
   SetStatusText( wxT(""), 2 );

   // Set the frame icon
   SetIcon(wxICON(mShogiIcon));

   mpEngine = new Engine;
      
   // Initialize children
   mpBoardCanvas = new BoardCanvas(this, mpEngine);

   int width, height;
   mpBoardCanvas->GetSize(&width, &height);
   SetBackgroundColour(mShogiApp::PiecesBackgroundColour());
   SetClientSize(width, height);

   // Obtain a list of the available games
   vector<string> gamesList;
   vector<string>::size_type i;
   mpEngine->GetGameList(gamesList);
   wxString* gamesArray = new wxString [ gamesList.size() ];
   for (i=0; i<gamesList.size(); i++) { 
      gamesArray[i] = wxString(gamesList[i].c_str(), wxConvLibc); 
   }

   // Create the game setup dialog
   mpGameSetupDlg = new GameSetupDlg(this, wxT("New game"), 
                                     gamesArray, gamesList.size());

   // Create the game setup dialog
   mpGameInfoDlg = new GameInfoDlg(this, wxT("New game"));

   // Clean up our temporary games array
   delete [] gamesArray;

   // Did we pass in a file name for a game to open?
   if (wxGetApp().argc == 2) {
      string name(wxString(wxGetApp().argv[1]).mb_str());
      string message;

      if (!mpEngine->LoadGame(name, message)) {
         wxString msg( message.c_str(), wxConvLibc );
         wxMessageBox(msg, wxT("Failure loading game"), 
                      wxOK |wxICON_ERROR, this);      
      }
      else {
         // Update the Board Canvas
         mpBoardCanvas->UpdateCanvas();
   
         int width, height;
         mpBoardCanvas->GetSize(&width, &height);
         SetClientSize(width, height);
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  ~mShogiFrame
// Description:  Destroys this instance of a mShogiFrame
//--------------------------------------------------------------------------
mShogiFrame::~mShogiFrame()
{
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  CreateTheMenuBar
// Description:  Creates the menu bar for mShogiFrame
//--------------------------------------------------------------------------
void mShogiFrame::CreateTheMenuBar()
{
#ifdef __WXMAC__
   wxApp::s_macAboutMenuItemId = ID_ABOUT;
#endif
   // Initialize the menu bar proper
   SetMenuBar( MyMenuBarFunc() );

   wxMenuBar* menubar = GetMenuBar();
   wxMenuItem* selection;

   // Initialize check items
   mNotationStyle = ID_ALGEBRAIC;
   selection = menubar->FindItem(mNotationStyle);
   selection->Check(TRUE);
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  CreateTheToolBar
// Description:  Creates the tool bar for mShogiFrame
//--------------------------------------------------------------------------
void mShogiFrame::CreateTheToolBar()
{
   wxToolBar *tb = CreateToolBar( wxTB_HORIZONTAL|wxNO_BORDER /* |wxTB_FLAT */ );

   // Initialize the tool bar proper
   MyToolBarFunc( tb );
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnLoad
// Description:  Load a saved game from a file
//--------------------------------------------------------------------------
void mShogiFrame::OnLoad( wxCommandEvent &event )
{
   wxFileDialog dialog( this, _T("File Open"), _T(""), _T(""),
                        _T("PGN files (*.pgn)|*.pgn"), 
                        wxOPEN|wxCHANGE_DIR);
   
   if (dialog.ShowModal() == wxID_OK) {
      string message;
      bool noErrors = mpEngine->LoadGame(string(dialog.GetPath().mb_str()), message);
      
      // Update the Board Canvas
      mpBoardCanvas->UpdateCanvas();

      int width, height;
      mpBoardCanvas->GetSize(&width, &height);
      SetClientSize(width, height);      
      if (!noErrors) {            
         wxString msg( message.c_str(), wxConvLibc );
         wxMessageBox(msg, wxT("Failure loading game"), 
                      wxOK |wxICON_ERROR, this);      
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnSave
// Description:  Save a game to a file
//--------------------------------------------------------------------------
void mShogiFrame::OnSave( wxCommandEvent &event )
{
   wxFileDialog dialog( this, _T("File Save"), _T(""), _T(""),
                        _T("PGN files (*.pgn)|*.pgn"), 
                        wxSAVE|wxCHANGE_DIR|wxOVERWRITE_PROMPT);
   
   if (dialog.ShowModal() == wxID_OK) {
      mpEngine->SaveGame(string(dialog.GetPath().mb_str()));
   }
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnQuit
// Description:  Quits the application
//--------------------------------------------------------------------------
void mShogiFrame::OnQuit( wxCommandEvent &event )
{
   Close(TRUE);
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnCloseWindow
// Description:  Closes the window and prepares the application for exit
//--------------------------------------------------------------------------
void mShogiFrame::OnCloseWindow( wxCloseEvent &event )
{
   // if ! saved changes -> return
   
   Destroy();
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnNew
// Description:  Opens up a dialog where the user can set up 
//               and start a new game
//--------------------------------------------------------------------------
void mShogiFrame::OnNew( wxCommandEvent &event )
{
   // Show the user a dialog to set up a new game
   mpGameSetupDlg->Centre();
   int result = mpGameSetupDlg->ShowModal();

   // Don't do anything if the user cancelled
   if (result != wxID_CANCEL) {
      // Create a new game based upon the selected settings
      string gameName = (const char*)(
         mpGameSetupDlg->GetLstGame()->GetStringSelection().mb_str());

      if (gameName != "") {
         // Change games
         mpEngine->SetGame(gameName);
      }

      // Start a fresh game
      mpEngine->RestartGame();

      // Update the Board Canvas
      mpBoardCanvas->UpdateCanvas();
   
      int width, height;
      mpBoardCanvas->GetSize(&width, &height);
      SetClientSize(width, height);

      // Update the player settings
      mpBoardCanvas->PlayerType(BLACK) =
         mpGameSetupDlg->GetRdbBplayer()->GetSelection();
      mpBoardCanvas->PlayerType(WHITE) = 
         mpGameSetupDlg->GetRdbWplayer()->GetSelection();

      // If the black player is the computer then
      // start thinking with the computer's thread
   }
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnGameInformation
// Description:  Opens up a dialog where the user can set up 
//               game information such as player names, game title, etc.
//--------------------------------------------------------------------------
void mShogiFrame::OnGameInformation( wxCommandEvent &event )
{
   // Show the user a dialog to set up game information
   mpGameInfoDlg->Centre();

   // Don't do anything if the user cancelled
   if (mpGameInfoDlg->ShowModal() != wxID_CANCEL) {
   }
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnTakeBack
// Description:  Take back the last move
//--------------------------------------------------------------------------
void mShogiFrame::OnTakeBack( wxCommandEvent &event )
{
   mpEngine->UnmakeMove();
   
   // Update the Board Canvas
   mpBoardCanvas->UpdateCanvas();
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnForce
// Description:  Force computer to finish search and make a move
//--------------------------------------------------------------------------
void mShogiFrame::OnForce( wxCommandEvent &event )
{
   mpEngine->StopSearch();
}

//--------------------------------------------------------------------------
//       Class:  mShogiFrame
//      Method:  OnAbout
// Description:  Displays the about dialog
//--------------------------------------------------------------------------
void mShogiFrame::OnAbout( wxCommandEvent &event )
{
   wxString title(wxT("About mShogi"));
   wxDialog dialog(this, -1, title);
  
   InitAboutDlg(&dialog, TRUE);
   
   dialog.Centre();
   dialog.ShowModal();
}


