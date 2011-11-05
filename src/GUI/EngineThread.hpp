////////////////////////////////////////////////////////////////////////////
// Name:         EngineThread.hpp
// Description:  Class that represents a worker thread for the mShogi 
//               engine's search for a good move
// Created:      08/26/2004 05:11:16 Eastern Standard Time
// Last Updated: $Date: 2004/08/28 02:50:49 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __EngineThread_HPP__
#define __EngineThread_HPP__

// Forward declarations
class Engine;
class BoardCanvas;

class EngineThread : public wxThread
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   EngineThread(BoardCanvas* canvas, Engine* engine);
   
   // ======================================================================
   // Call Back Methods
   // ======================================================================
   virtual void* Entry();
   virtual void OnExit();
   
   // ======================================================================
   // Constants
   // ======================================================================
   enum {
      ENGINE_EVENT = 201
   };

private:
   // ======================================================================
   // Data
   // ======================================================================
   BoardCanvas* mpBoardCanvas; // Pointer to the board canvas
   Engine*      mpEngine;      // Pointer to the game engine
};

#endif

