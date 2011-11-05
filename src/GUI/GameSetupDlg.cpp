////////////////////////////////////////////////////////////////////////////
// Name:         GameSetupDlg.cpp
// Description:  Dialog for setting up a game in mShogi
// Created:      08/12/2004 10:58:16 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 21:54:58 $
// Revision:     $Revision: 1.2 $
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
#include "GameSetupDlg.hpp"

// WDR: class implementations

//----------------------------------------------------------------------------
// GameSetupDlg
//----------------------------------------------------------------------------

// ============================================================================
// Event Tables
// ============================================================================

// WDR: event table for GameSetupDlg

BEGIN_EVENT_TABLE(GameSetupDlg,wxDialog)
   EVT_BUTTON( ID_BTN_FLIP, GameSetupDlg::OnPressFlipButton )
END_EVENT_TABLE()

//--------------------------------------------------------------------------
//       Class:  GameSetupDlg
//      Method:  GameSetupDlg
// Description:  Construct an instance of a GameSetupDlg object
//--------------------------------------------------------------------------
GameSetupDlg::GameSetupDlg( wxWindow *parent, const wxString &title,
                            wxString* items,  int nItems)
: wxDialog( parent, -1, title, wxDefaultPosition, wxDefaultSize, 
            wxDEFAULT_DIALOG_STYLE )
{
   // WDR: dialog function InitSetupNotebook for GameSetupDlg
   InitSetupNotebook( this, TRUE ); 
   
   // Populate the game list box with game types
   GetLstGame()->InsertItems(nItems, items, 0);

   // Initialize player radio boxes
   GetRdbBplayer()->SetSelection(HUMAN);
   GetRdbWplayer()->SetSelection(HUMAN);
   
   // Initialize data
   mBoardFlipped = false;
}

//------------------------------------------------------------------------
//       Class:  GameSetupDlg
//      Method:  OnPressFlipButton
// Description:  Flip board sample board image and set flag to indicate
//               whether or not the standard board image should be
//               flipped or not
//------------------------------------------------------------------------
void GameSetupDlg::OnPressFlipButton( wxCommandEvent &event )
{
   // Toggle the board flag
   mBoardFlipped = !mBoardFlipped;

   // Retrieve the static bitmap control for the board image
   wxStaticBitmap* bmpFlipBoard = GetBmpFlipboard();
    
   // Create an image from the bitmap of the current board
   wxImage image = bmpFlipBoard->GetBitmap().ConvertToImage();
    
   // Rotate the image about its center 180 degrees
   image = image.Rotate90();
   wxBitmap bmp = wxBitmap(image.Rotate90());
            
   // Replace the old bitmap with this new one
   bmpFlipBoard->SetBitmap(bmp);
   bmpFlipBoard->Refresh();
}

// WDR: handler implementations for GameSetupDlg

