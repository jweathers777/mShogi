////////////////////////////////////////////////////////////////////////////
// Name:         EngineThread.cpp
// Description:  Class that represents a worker thread for the mShogi 
//               engine's search for a good move
// Created:      08/26/2004 05:11:17 Eastern Standard Time
// Last Updated: $Date: 2004/08/28 02:50:49 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// =========================================================================
// Header file includes
// =========================================================================

// Standard C++ header files

// STL header files
#include <string>
#include <vector>

// wxWidgets header files
#include "wx/wx.h"
#include "wx/thread.h"

// Local header files
#include "BoardCanvas.hpp"
#include "EngineThread.hpp"
#include "Engine.hpp"

using std::string;

//--------------------------------------------------------------------------
//       Class:  EngineThread
//      Method:  EngineThread
// Description:  Construct an instance of a EngineThread object
//--------------------------------------------------------------------------
EngineThread::EngineThread(BoardCanvas* canvas, Engine* engine)
{
   mpBoardCanvas = canvas;
   mpEngine = engine;
}

//------------------------------------------------------------------------
//       Class:  EngineThread
//      Method:  Entry
// Description:  The entry point for thread execution 
//------------------------------------------------------------------------
void* EngineThread::Entry()
{
   string move;
   
   // Commence the search
   mpEngine->StartSearching();
   
   bool succeeded = mpEngine->MakeBestMove(mpEngine->GetSide(), move);
         
   // Finish searching
   mpEngine->FinishSearching();

   // Inform the GUI that we are finished
   wxCommandEvent event(wxEVT_COMMAND_MENU_SELECTED, ENGINE_EVENT);
   event.SetInt((int)succeeded);
   wxPostEvent(mpBoardCanvas, event);
   
   return 0;
}

//------------------------------------------------------------------------
//       Class:  EngineThread
//      Method:  OnExit
// Description:  Called when the thread exits normally or is deleted
//------------------------------------------------------------------------
void EngineThread::OnExit()
{
}


