////////////////////////////////////////////////////////////////////////////
// Name:         BoardCanvas.hpp
// Description:  Canvas window for displaying the current board
// Created:      08/12/2004 11:08:21 Eastern Standard Time
// Last Updated: $Date: 2004/09/30 02:03:53 $
// Revision:     $Revision: 1.2 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __BoardCanvas_HPP__
#define __BoardCanvas_HPP__

// Local header file includes
#include "PieceInfo.hpp"

// Forward declarations
class Engine;
class EngineThread;
class wxDragImage;

class BoardCanvas : public wxWindow
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   BoardCanvas(wxWindow* parent, Engine* engine);
   virtual ~BoardCanvas();

   // ======================================================================
   // Access
   // ======================================================================
   int& PlayerType(int side) { return mPlayerType[side]; }
   
   // ======================================================================
   // Operations
   // ======================================================================
   void UpdateCanvas(bool init=false);
   
private:
   void DrawPiece(wxDC& dc, int boardindex, int imageindex, bool rotated);
   void DrawPieces(wxDC& dc);
   void ErasePiece(wxDC& dc, int index);

   void DrawLabels();

   void HiLightSquares(const std::vector<int>& squares, bool isOpponent=false);
   
   void LoadMappingData(const wxString& filename);
   
   std::string CreateMoveString(int start, int finish);
   std::string CreateMoveString(int start, int middle, int finish);
   
   int FindSquare(const wxPoint& point);
   wxPoint GetUpperLeftCorner(int index);

   // ======================================================================
   // Event Handlers
   // ======================================================================
   void OnEraseBackground(wxEraseEvent& event);
   
   void OnMouseLeftDown(wxMouseEvent& event);
   void OnMouseMotion(wxMouseEvent& event);
   void OnMouseLeftUp(wxMouseEvent& event);

   void OnEngineEvent(wxCommandEvent& event);

private:
   // ======================================================================
   // Constants
   // ======================================================================
   static const int NO_PROMOTION; // Constant that indicates no promotion
   static const int BORDER_WIDTH; // Width in pixels of board border
   
   // Drag constants
   enum {
      DRAG_NONE = 0,
      DRAG_ILLEGAL,
      DRAG_START,
      DRAG_IN_PROGRESS
   };

   // Promoted State constants
   enum {
      UNPROMOTED = 0,
      PROMOTED
   };

   // Rotated state constants
   enum {
      NORMAL = 0,
      ROTATED
   };

   // ======================================================================
   // Widgets
   // ======================================================================
   wxBitmap* mpCanvasBitmap;  // Composite image displayed in canvas
   wxBitmap* mpBoardBitmap;   // Board image for the current game
   
   wxBitmap* mpPiecesBitmap[2];  // Pieces array images for the current game

   wxCursor* mpOpenHandCursor;   // Open hand cursor
   wxCursor* mpClosedHandCursor; // Closed hand cursor

   wxDragImage* mpDragImage;    // The current drag image
      
   // ======================================================================
   // Data
   // ======================================================================
   int mBoardColumns;  // Number of columns on the current board
   int mBoardRows;     // Number of rows on the current board
   
   int mImageColumns;  // Number of columns in the pieces array image
   int mImageRows;     // Number of rows in the pieces array image

   int mSquareWidth;   // Width of a square in pixels
   int mSquareHeight;  // Height of a square in pixels

   bool mDrawLabels;   // Indicates whether we should draw the square labels

   std::vector<int>    mImageMap[2];   // Maps piece types to image indices
   
   std::vector<PieceInfo> mPieceInfo;  // Vector of current pieces on board

   std::vector<int> mSquares;  // Destination squares that need to be
                               // Hi-Lighted or redrawn

   bool       mPassMove;          // A pass move is available
   int        mPieceUnderCursor;  // Piece type of the piece under the cursor
   
   int        mDragState;         // The state of dragging with the mouse
   int        mStartIndex;        // Index of the drag's starting square
   int        mPriorIndex;        // Two-part move's starting index square
   int        mDisplayIndex;      // Two-part move's starting index square
   wxPoint    mStartPosition;     // The drag's starting position

   Engine*    mpEngine;           // Pointer to the game engine

   int mPlayerType[2];            // The type of players playing
   
   DECLARE_EVENT_TABLE()
};

#endif

