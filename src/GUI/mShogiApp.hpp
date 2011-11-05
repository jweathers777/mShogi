////////////////////////////////////////////////////////////////////////////
// Name:         mShogiApp.hpp
// Description:  wxWidgets Application class for mShogi
// Created:      08/10/2004 04:08:47 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 02:50:49 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __mShogiApp_HPP__
#define __mShogiApp_HPP__

class mShogiApp : public wxApp
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   mShogiApp();
   ~mShogiApp();

   // ======================================================================
   // Operations
   // ======================================================================
   virtual bool OnInit();
   wxString GetExecutablePath();

   // ======================================================================
   // Static Operations
   // ======================================================================
   static const wxColour& PiecesBackgroundColour();
   static const wxBrush& PiecesBackgroundBrush();
   
private:
   // ======================================================================
   // Static values
   // ======================================================================
   static wxColour* mpPiecesBackgroundColour;
   static wxBrush*  mpPiecesBackgroundBrush;
};

DECLARE_APP(mShogiApp)

#endif
