////////////////////////////////////////////////////////////////////////////
// Name:         mShogiApp.hpp
// Description:  wxWidgets Application class for mShogi
// Created:      08/10/2004 04:10:24 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 02:50:49 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// ===========================================================================
// Header file includes
// ===========================================================================

// wxWidgets header files
#include "wx/wx.h"
#include "wx/filename.h"
#include "wx/colour.h"
#include "wx/brush.h"

// Local header files
#include "mShogiFrame.hpp"
#include "mShogiApp.hpp"
#include "GameSetupDlg.hpp"

wxColour* mShogiApp::mpPiecesBackgroundColour = 0;
wxBrush*  mShogiApp::mpPiecesBackgroundBrush = 0;

IMPLEMENT_APP(mShogiApp)

//--------------------------------------------------------------------------
//       Class:  mShogiApp
//      Method:  mShogiApp
// Description:  Construct an instance of an mShogiApp object
//--------------------------------------------------------------------------
mShogiApp::mShogiApp()
{
}

//--------------------------------------------------------------------------
//       Class:  mShogiApp
//      Method:  ~mShogiApp
// Description:  Destroys an instance of an mShogiApp object
//--------------------------------------------------------------------------
mShogiApp::~mShogiApp()
{
   delete mpPiecesBackgroundColour;
   delete mpPiecesBackgroundBrush;
}

//--------------------------------------------------------------------------
//       Class:  mShogiApp
//      Method:  OnInit
// Description:  Initialize the application
//--------------------------------------------------------------------------
bool mShogiApp::OnInit()
{
   // Create the main window frame
   mShogiFrame *frame = new mShogiFrame( NULL, -1, wxT("mShogi"),
                                         wxDefaultPosition, wxDefaultSize );
   frame->Centre();
   frame->Show( TRUE );
   frame->SetFocus();
    
   return TRUE;
}

//------------------------------------------------------------------------
//       Class:  mShogiApp
//      Method:  PiecesBackgroundColour
// Description:  Return a reference to the pieces image background colour
//------------------------------------------------------------------------
const wxColour& mShogiApp::PiecesBackgroundColour()
{
   if (!mpPiecesBackgroundColour) {
      mpPiecesBackgroundColour = new wxColour(239, 181, 114);
   }

   return *mpPiecesBackgroundColour;
}

//------------------------------------------------------------------------
//       Class:  mShogiApp
//      Method:  PiecesBackgroundBrush
// Description:  Return a reference to the pieces image background brush
//------------------------------------------------------------------------
const wxBrush& mShogiApp::PiecesBackgroundBrush()
{
   if (!mpPiecesBackgroundBrush) {
      mpPiecesBackgroundBrush = new wxBrush(PiecesBackgroundColour(), wxSOLID);
   }

   return *mpPiecesBackgroundBrush;
}

