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

//--------------------------------------------------------------------------
//       Class:  mShogiApp
//      Method:  GetExecutablePath
// Description:  Returns the path to the application's executable
//      Author:  Nigel Hathaway
//      Source:  Taken from wxWidgets website article on writing
//               installers for wxWidgets applications
//--------------------------------------------------------------------------
wxString mShogiApp::GetExecutablePath()
{
   static bool found = false;
   static wxString path;

   if (found)
      return path;
   else
   {
#ifdef __WXMSW__

      char buf[512];
      *buf = '\0';
      GetModuleFileName(NULL, buf, 511);
      path = buf;

#elif defined(__WXMAC__)

      ProcessInfoRec processinfo;
      ProcessSerialNumber procno ;
      FSSpec fsSpec;

      procno.highLongOfPSN = NULL ;
      procno.lowLongOfPSN = kCurrentProcess ;
      processinfo.processInfoLength = sizeof(ProcessInfoRec);
      processinfo.processName = NULL;
      processinfo.processAppSpec = &fsSpec;

      GetProcessInformation( &procno , &processinfo ) ;
      path = wxMacFSSpec2MacFilename(&fsSpec);
#else
      wxString argv0 = this->argv[0];

      if (wxIsAbsolutePath(argv0))
         path = argv0;
      else
      {
         wxPathList pathlist;
         pathlist.AddEnvList(wxT("PATH"));
         path = pathlist.FindAbsoluteValidPath(argv0);
      }

      wxFileName filename(path);
      filename.Normalize();
      path = filename.GetFullPath();
#endif
      found = true;
      return path;
   }
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

