////////////////////////////////////////////////////////////////////////////
// Name:         BoardCanvas.cpp
// Description:  Canvas window for displaying the current board
// Created:      08/12/2004 11:08:21 Eastern Standard Time
// Last Updated: $Date: 2004/09/30 02:03:53 $
// Revision:     $Revision: 1.3 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// ===========================================================================
// Header file includes
// ===========================================================================

// wxWidget header files
#include "wx/wx.h"
#include "wx/filename.h"
#include "wx/image.h"
#include "wx/colour.h"
#include "wx/log.h"

// Under Windows, change this to 1
// to use wxGenericDragImage

#define wxUSE_GENERIC_DRAGIMAGE 1

#if wxUSE_GENERIC_DRAGIMAGE
#include "wx/generic/dragimgg.h"
#define wxDragImage wxGenericDragImage
#else
#include "wx/dragimag.h"
#endif

// Standard C++ header files
#include <iostream>
#include <fstream>
#include <sstream>

// STL header files
#include <vector>
#include <map>
#include <string>
#include <algorithm>
#include <utility>

// Local header files
#include "common.hpp"
#include "mShogiApp.hpp"
#include "BoardCanvas.hpp"
#include "EngineThread.hpp"
#include "Engine.hpp"

using std::ios;
using std::ifstream;
using std::ofstream;
using std::istringstream;
using std::ostringstream;
using std::endl;
using std::map;
using std::make_pair;
using std::vector;
using std::string;

// Static data initialization
const int BoardCanvas::NO_PROMOTION = -1;
const int BoardCanvas::BORDER_WIDTH = 17;

// ============================================================================
// Event Tables
// ============================================================================
BEGIN_EVENT_TABLE(BoardCanvas,wxWindow)
   EVT_ERASE_BACKGROUND(BoardCanvas::OnEraseBackground)
   EVT_LEFT_DOWN(BoardCanvas::OnMouseLeftDown)
   EVT_MOTION(BoardCanvas::OnMouseMotion)
   EVT_LEFT_UP(BoardCanvas::OnMouseLeftUp)
   EVT_MENU(EngineThread::ENGINE_EVENT, BoardCanvas::OnEngineEvent)
END_EVENT_TABLE()

//--------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  BoardCanvas
// Description:  Construct an instance of a BoardCanvas window
//--------------------------------------------------------------------------
BoardCanvas::BoardCanvas(wxWindow* parent, Engine* engine)
: wxWindow(parent, -1, wxPoint(0,0), wxSize(-1,-1), wxSUNKEN_BORDER)
{
   // Initialize our hand cursors
#ifdef __WXMSW__
   mpOpenHandCursor = new wxCursor("OpenHandCursor", 
                                   wxBITMAP_TYPE_CUR_RESOURCE);
   mpClosedHandCursor = new wxCursor("ClosedHandCursor",
                                   wxBITMAP_TYPE_CUR_RESOURCE);
#else
   mpOpenHandCursor = new wxCursor(wxCURSOR_ARROW);
   mpClosedHandCursor = new wxCursor(wxCURSOR_HAND);
#endif

   SetCursor(*mpOpenHandCursor);

   mPieceUnderCursor = -1;

   mDrawLabels = true;
   mPassMove = false;
   mPriorIndex = -1;
   
   // Initialize engine pointer
   mpEngine = engine;
   
   // Intialize pointers to null values
   mpBoardBitmap = 0;
   mpCanvasBitmap = 0;
   mpDragImage = 0;
   mpPiecesBitmap[NORMAL] = 0;
   mpPiecesBitmap[ROTATED] = 0;

   // Initialize player types
   mPlayerType[BLACK] = HUMAN;
   mPlayerType[WHITE] = HUMAN;

   // Perform initial update of the canvas to initialize things
   UpdateCanvas(true);
}

//--------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  ~BoardCanvas
// Description:  Destroys this instance of a BoardCanvas window
//--------------------------------------------------------------------------
BoardCanvas::~BoardCanvas()
{
   delete mpOpenHandCursor;
   delete mpClosedHandCursor;

   if (mpDragImage) delete mpDragImage;
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  UpdateCanvas
// Description:  Update the image, size, and position of the canvas
//               based upon a change of games
//------------------------------------------------------------------------
void BoardCanvas::UpdateCanvas(bool init)
{
   // What game are we currently playing?
   wxString gameName(mpEngine->GetGame().c_str(), wxConvLibc);

   // Obtain the full path to the current game's installation files
   wxFileName applicationExePath(wxGetApp().GetExecutablePath());
   wxFileName gameFilesPath(applicationExePath.GetPath(), gameName);
   
   // Construct sources for board image, pieces image, and setup data
   wxFileName boardSrc(gameFilesPath.GetFullPath(),   wxT("board.bmp"));
   wxFileName piecesSrc(gameFilesPath.GetFullPath(),  wxT("pieces.bmp"));
   wxFileName mappingSrc(gameFilesPath.GetFullPath(), wxT("piecemap.dat"));


   // Make sure these files exist before we proceed
   if (!boardSrc.FileExists()) {
      wxLogFatalError(wxT("The file %s does not exist! Your installation has "
                      "become corrupt. Please reinstall mShogi."),
                      boardSrc.GetFullPath().c_str());
   }
   if (!piecesSrc.FileExists()) {
      wxLogFatalError(wxT("The file %s does not exist! Your installation has "
                      "become corrupt. Please reinstall mShogi."),
                      piecesSrc.GetFullPath().c_str());
   }
   if (!mappingSrc.FileExists()) {
      wxLogFatalError(wxT("The file %s does not exist! Your installation has "
                      "become corrupt. Please reinstall mShogi."),
                      mappingSrc.GetFullPath().c_str());
   }

   // Create a new board bitmap
   if (mpBoardBitmap) delete mpBoardBitmap;
   mpBoardBitmap = new wxBitmap(boardSrc.GetFullPath(), wxBITMAP_TYPE_BMP);
   int width = mpBoardBitmap->GetWidth();
   int height = mpBoardBitmap->GetHeight();

   // Resize, position, and set the background colour for the canvas
   SetClientSize(width, height);
   Centre();

   wxImage image(piecesSrc.GetFullPath(), wxBITMAP_TYPE_BMP);
   image.SetMaskColour(mShogiApp::PiecesBackgroundColour().Red(),
                       mShogiApp::PiecesBackgroundColour().Green(),
                       mShogiApp::PiecesBackgroundColour().Blue());
   
   if (mpPiecesBitmap[NORMAL]) delete mpPiecesBitmap[NORMAL];
   mpPiecesBitmap[NORMAL] = new wxBitmap(image);

   image = image.Mirror(TRUE);
   image = image.Mirror(FALSE);
   image.SetMaskColour(mShogiApp::PiecesBackgroundColour().Red(),
                       mShogiApp::PiecesBackgroundColour().Green(),
                       mShogiApp::PiecesBackgroundColour().Blue());

   if (mpPiecesBitmap[ROTATED]) delete mpPiecesBitmap[ROTATED];
   mpPiecesBitmap[ROTATED] = new wxBitmap(image);
   
   
   // Initialize members that describe the dragging process
   mDragState = DRAG_NONE;
   if (mpDragImage) delete mpDragImage;
   mpDragImage = 0;
   
   // Load the game specific setup data
   mPieceInfo.clear();
   mImageMap[NORMAL].clear();
   mImageMap[ROTATED].clear();
   LoadMappingData(mappingSrc.GetFullPath());

   // Draw the square labels on the board if necessary
   if (mDrawLabels) DrawLabels();

   // Initialize canvas bitmap data
   if (mpCanvasBitmap) delete mpCanvasBitmap;
   mpCanvasBitmap = new wxBitmap(width, height);
   wxMemoryDC memDC;
   memDC.SelectObject(*mpCanvasBitmap);
   memDC.DrawBitmap(*mpBoardBitmap, 0, 0);
   DrawPieces(memDC);
   memDC.SelectObject(wxNullBitmap);

   if (!init) Refresh();
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  DrawPiece
// Description:  Draw the piece given its square index on the board 
//               and its image index in the pieces image
//------------------------------------------------------------------------
void BoardCanvas::DrawPiece(wxDC& dc, int boardindex, 
                            int imageindex, bool rotated)
{
   // Adjust image coordinates for a rotated image
   if (rotated) {
      imageindex = (mImageColumns * mImageRows) - 1 - imageindex;
   }

   int column = imageindex % mImageColumns;
   int row    = imageindex / mImageColumns;
   
   int sourceX = column * mSquareWidth;
   int sourceY = row    * mSquareHeight;

   column = mBoardColumns - 1 - (boardindex % mBoardColumns);
   row    = boardindex / mBoardColumns;
   
   int destX = column * (mSquareWidth + 1) + BORDER_WIDTH + 1;
   int destY = row   * (mSquareHeight + 1) + BORDER_WIDTH + 1;
   
   wxBitmap* srcBitmapPtr = rotated ? mpPiecesBitmap[ROTATED] 
                                    : mpPiecesBitmap[NORMAL];

   wxRect rect(sourceX, sourceY, mSquareWidth, mSquareHeight);
   dc.DrawBitmap(srcBitmapPtr->GetSubBitmap(rect), destX, destY, TRUE);
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  DrawPieces
// Description:  Draws the pieces on the canvas
//------------------------------------------------------------------------
void BoardCanvas::DrawPieces(wxDC& dc)
{
   vector<PieceInfo>::size_type index;
   int piecetype, imageindex;
   bool rotated;
  
   // Fetch the current state of the board
   mpEngine->GetPieceInfo(mPieceInfo);
   
   // Redraw the pieces on the board
   for (index = 0; index < mPieceInfo.size(); index++) {
      piecetype = mPieceInfo[index].mType;
      if (piecetype != NO_PIECE) {
         if (mPieceInfo[index].mPromoted) {
            imageindex = mImageMap[PROMOTED][piecetype];
         }
         else {
            imageindex = mImageMap[UNPROMOTED][piecetype];
         }
         rotated = (mPieceInfo[index].mSide == WHITE);
         if (index != mPriorIndex) {
            DrawPiece( dc, index, imageindex, rotated );
         }
         else {
            DrawPiece( dc, mDisplayIndex, imageindex, rotated );
         }
      }
   }
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  ErasePiece
// Description:  Erase any piece on the square associated 
//               with the given square index
//------------------------------------------------------------------------
void BoardCanvas::ErasePiece(wxDC& dc, int index)
{
   int column = mBoardColumns - 1 - (index % mBoardColumns);
   int row = index / mBoardColumns;
      
   int x = column * (mSquareWidth + 1) + BORDER_WIDTH + 1;
   int y = row   * (mSquareHeight + 1) + BORDER_WIDTH + 1;

   dc.SetClippingRegion(x, y, mSquareWidth, mSquareHeight);
   dc.DrawBitmap(*mpBoardBitmap, 0, 0);
   dc.DestroyClippingRegion();
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  DrawLabels
// Description:  Draws square labels on the base board bitmap
//------------------------------------------------------------------------
void BoardCanvas::DrawLabels()
{
   char label;
   int x, y;
   
   // Prepare device context for drawing to the bitmap
   wxMemoryDC memDC;
   memDC.SelectObject(*mpBoardBitmap);
   memDC.SetTextBackground(mShogiApp::PiecesBackgroundColour());
   memDC.SetTextForeground(*wxBLACK);

   // Calculate an appropriate font size
   wxFont font(1, wxDEFAULT, wxNORMAL, wxBOLD, FALSE, wxT("Sans"));
   wxString test = wxT("abcdefghijklmnopqrstuvwxyz1234567890");
   int fontsize = 1;
   int textwidth, textheight;
   bool finished = false;

   while (!finished) {
      font.SetPointSize(fontsize);
      memDC.SetFont(font);
      memDC.GetTextExtent(test, &textwidth, &textheight);
      
      if ((3*textwidth <= 72*BORDER_WIDTH) && 
          (3*textheight <= 2*BORDER_WIDTH)) {
         fontsize++;
      }
      else {
         finished = true;
         fontsize--;
      }
   }

   int shift;
   // Draw the vertical labels
   x = mBoardColumns * (mSquareWidth + 1) + BORDER_WIDTH + 1;
   for (int row = 0; row < mBoardRows; row++) {
      label = (char)((int)'a' + row);
      wxString strLabel(&label, wxConvUTF8);
      memDC.GetTextExtent(strLabel, &textwidth, &textheight);
      shift = (mSquareHeight - textheight) / 2;
      y = row * (mSquareHeight + 1) + BORDER_WIDTH + 1 + shift;
      shift = (BORDER_WIDTH - textwidth) / 2;
      memDC.DrawText(strLabel, x + shift, y);
   }
 
   // Draw the horizontal labels
   shift = (BORDER_WIDTH/6);
   y = mBoardRows * (mSquareHeight + 1) + BORDER_WIDTH + 1 + shift;
   for (int col = 0; col < mBoardColumns; col++) {
      ostringstream output;
      output << (col+1);
      wxString strLabel(output.str().c_str(), wxConvUTF8);
      memDC.GetTextExtent(strLabel, &textwidth, &textheight);
      shift = (mSquareWidth - textwidth) / 2;
      x = (mBoardColumns - 1 - col)* (mSquareWidth + 1) + 
          BORDER_WIDTH + 1 + shift;
      memDC.DrawText(strLabel, x, y);
   }
   
   // Disconnect the device context from our bitmap
   memDC.SelectObject(wxNullBitmap);
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  HiLightSquares
// Description:  Hi-Lights the squares indicated in the square list
//------------------------------------------------------------------------
void BoardCanvas::HiLightSquares(const vector<int>& squares, bool isOpponent)
{
   const wxPen* penPtr = 0;
   const wxBrush* brushPtr = 0;
   // Prepare to draw to the canvas bitmap
   wxMemoryDC memDC;
   memDC.SelectObject(*mpCanvasBitmap);
   if (isOpponent) {
      penPtr = wxRED_PEN;
      brushPtr = wxRED_BRUSH;
   }
   else {
      penPtr = wxGREEN_PEN;
      brushPtr = wxGREEN_BRUSH;
   }

   unsigned i;
   vector<bool> visited(mBoardColumns*mBoardRows, false);

   mPassMove = false;
   
   int boardindex, column, row, x, y;
   int radius = mSquareWidth / 4;
   for (i = 0; i < squares.size(); i++) {
      if (boardindex == mStartIndex) {
         mPassMove = true;
      }
      // Calculate the center of the current square
      boardindex = squares[i];
      if (boardindex >= 0) {
         if (visited[boardindex]) continue;

         memDC.SetPen(*penPtr);
         memDC.SetBrush(*brushPtr);
      }
      else {
         boardindex *= -1;
         if (visited[boardindex]) continue;
         
         visited[boardindex] = true;
         memDC.SetPen(*wxCYAN_PEN);
         memDC.SetBrush(*wxCYAN_BRUSH);
      }
      column = mBoardColumns - 1 - (boardindex % mBoardColumns);
      row    = boardindex / mBoardColumns;
      x = (2*column + 1) * (mSquareWidth + 1) / 2 + BORDER_WIDTH + 1;
      y = (2*row + 1) * (mSquareHeight + 1) / 2 + BORDER_WIDTH + 1;

      // Draw a circle in the center of the square
      memDC.DrawCircle(x, y, radius);
   }
   
   memDC.SelectObject(wxNullBitmap);

   // Redisplay the canvas on the screen
   wxClientDC dc(this);
   dc.DrawBitmap(*mpCanvasBitmap, 0, 0);
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  LoadMappingData
// Description:  Loads from a file the mappings between piece constants
//               and piece subimages in the pieces image file
//------------------------------------------------------------------------
void BoardCanvas::LoadMappingData(const wxString& filename)
{
   char junk;
   int size;
   
   ifstream input(filename.mb_str());
   string line;

   // Extract board dimensions in squares
   input >> mBoardColumns >> junk >> mBoardRows;
   
   // Extract square dimensions in pixels
   input >> mSquareWidth >> junk >> mSquareHeight;

   // Extract piece image dimensions in squares
   input >> mImageColumns >> junk >> mImageRows; 
   
   // Extract number of piece constants
   input >> size;

   // Allocate image map vectors to this size
   mImageMap[UNPROMOTED].resize(size);
   mImageMap[PROMOTED].resize(size);
   
   // Skip the end of the line
   input.get(junk);
   
   // Extract piece information
   int type, promoted_type, row, column, index;
   
   while (!input.eof()) {
      getline(input, line);
      if (line == "") continue;

      // Replace commas with spaces for ease of parsing
      replace(line.begin(), line.end(), ',', ' ');
      istringstream linestream(line);

      // Extract piece type constant
      // and promoted piece type constant
      linestream >> type >> promoted_type;

      // Extract row and column numbers for the piece image
      linestream >> row >> column;
      index = column + row*mImageColumns;

      // Map the piece type to its image index
      mImageMap[UNPROMOTED][type] = index;
      
      // Map the piece type to its promoted image index
      mImageMap[PROMOTED][type] = NO_PROMOTION;
      
      // Does this piece promote?
      if (promoted_type != NO_PROMOTION) {
         // Extract row and column numbers for the piece image
         linestream >> row >> column;
         index = column + row*mImageColumns;
       
         // Map the piece type to its promoted image index
         mImageMap[PROMOTED][type] = index;
      }
   }
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  CreateMoveString
// Description:  Creates a move string from the start and finish 
//               squares' indices
//------------------------------------------------------------------------
string BoardCanvas::CreateMoveString(int start, int finish)
{
   ostringstream output;
   
   int column = start % mBoardColumns;
   int row    = start / mBoardColumns;
   
   output << (column+1) << (char)(row + (int)'a') << '-';
   
   column = finish % mBoardColumns;
   row    = finish / mBoardColumns;
   
   output << (column+1) << (char)(row + (int)'a');

   return output.str();
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  CreateMoveString
// Description:  Creates a move string from the start, middle, 
//               and finish squares' indices
//------------------------------------------------------------------------
string BoardCanvas::CreateMoveString(int start, int middle, int finish)
{
   ostringstream output;
   
   int column = start % mBoardColumns;
   int row    = start / mBoardColumns;
   
   output << (column+1) << (char)(row + (int)'a') << '-';
   
   if (middle != finish) {
      column = middle % mBoardColumns;
      row    = middle / mBoardColumns;
   
      output << (column+1) << (char)(row + (int)'a') << '-';
   }
   
   column = finish % mBoardColumns;
   row    = finish / mBoardColumns;
   
   output << (column+1) << (char)(row + (int)'a');

   return output.str();
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  FindSquare
// Description:  Takes the coordinates of a mouse event and returns the
//               index of the board square in which the mouse event occurred
//------------------------------------------------------------------------
int BoardCanvas::FindSquare(const wxPoint& point)
{
   int column = (point.x - BORDER_WIDTH - 1) / (mSquareWidth + 1);
   int row = (point.y - BORDER_WIDTH - 1) / (mSquareHeight + 1);

   if (column < 0 || column >= mBoardColumns ||
          row < 0 ||    row >= mBoardRows)
      return -1;

   column = mBoardColumns - 1 - column;

   return (column + row*mBoardColumns);
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  GetUpperLeftCorner
// Description:  Returns the upper left corner coordinates for the square
//               whose index number is passed into the method
//------------------------------------------------------------------------
wxPoint BoardCanvas::GetUpperLeftCorner(int index)
{
   int column = mBoardColumns - 1 - (index % mBoardColumns);
   int row = index / mBoardColumns;
      
   int x = column * (mSquareWidth + 1) + BORDER_WIDTH + 1;
   int y = row   * (mSquareHeight + 1) + BORDER_WIDTH + 1;

   return wxPoint(x,y);
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  OnEraseBackground
// Description:  Redraws the canvas
//------------------------------------------------------------------------
void BoardCanvas::OnEraseBackground(wxEraseEvent& event)
{
   if (mpCanvasBitmap) {
      if (event.GetDC()) {
         event.GetDC()->DrawBitmap(*mpCanvasBitmap, 0, 0);
      }
      else {
         wxClientDC dc(this);
         dc.DrawBitmap(*mpCanvasBitmap, 0, 0);
      }
   }
   else {
      event.Skip();
   }
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  OnMouseLeftDown
// Description:  Responds to pressing the left mouse button
//------------------------------------------------------------------------
void BoardCanvas::OnMouseLeftDown(wxMouseEvent& event)
{
   int side = mpEngine->GetSide();
   
   // First make sure that the game is not finished already
   if ( mpEngine->IsGameFinished() ) {
      wxString msg( mpEngine->GameStatusMessage().c_str(), wxConvLibc );
      wxMessageBox(msg, wxT("Game Over"), wxOK |wxICON_INFORMATION,
                   this->GetParent());
   }
   // Make sure it isn't the computer's turn
   else if (mPlayerType[side] == COMPUTER) {
      wxString msg( wxT("It is not your turn to move."));
      wxMessageBox(msg, wxT("Computer Thinking"), wxOK |wxICON_INFORMATION,
                   this->GetParent());
   }
   else {
      int index = FindSquare(event.GetPosition());
      if (mPriorIndex == -1) {
         if ( (index > -1) && mpEngine->IsPieceOnSquare(index) ) {
            if ( mpEngine->IsPieceColor(index, side) ) {
               // Set up data for dragging
               mDragState = DRAG_START;
               mStartPosition = event.GetPosition();
               mStartIndex = index;

               // Hi-light squares to which we may move
               mSquares.clear();
               mpEngine->GetDestinationSquares(index, mSquares);
               HiLightSquares(mSquares);
            }
            else {
               // Hi-light squares to which we may move
               mSquares.clear();
               mpEngine->GetDestinationSquares(index, mSquares);
               HiLightSquares(mSquares, true);
               
               mDragState = DRAG_ILLEGAL;
            }
         }
      }
      else if (index == mDisplayIndex) {
         // Set up data for dragging
         mDragState = DRAG_START;
         mStartPosition = event.GetPosition();
         mStartIndex = index;

         // Hi-light squares to which we may move
         mSquares.clear();
         mpEngine->GetSecondSquares(mPriorIndex, mDisplayIndex, mSquares);
         HiLightSquares(mSquares);
      }
   }
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  OnMouseMotion
// Description:  Responds to moving the mouse
//------------------------------------------------------------------------
void BoardCanvas::OnMouseMotion(wxMouseEvent& event)
{
   if (mDragState == DRAG_NONE) {
      int index = FindSquare(event.GetPosition());
      if (mPriorIndex != -1) {
         index = mDisplayIndex;
      }
      if ( (index > -1) && mpEngine->IsPieceOnSquare(index) ) {
         int piecetype = mPieceInfo[index].mType;
         if (piecetype != mPieceUnderCursor) {
            mPieceUnderCursor = piecetype;
            
            wxString name = wxString::Format(wxT("%s (%s)"),
                              mpEngine->GetPieceName(piecetype,1).c_str(),
                              mpEngine->GetPieceName(piecetype).c_str());
            wxString pname;
            if (mImageMap[PROMOTED][piecetype] == NO_PROMOTION) {
               pname = wxT("Does not promote");
            }
            else {
               pname.Printf(wxT("%s (%s)"),
                  mpEngine->GetPieceName(piecetype,1,true).c_str(),
                  mpEngine->GetPieceName(piecetype,0,true).c_str());
            }
            
            wxFrame* parentPtr = (wxFrame*)this->GetParent();
            parentPtr->SetStatusText(name, 1);
            parentPtr->SetStatusText(pname, 2);
         }
      }
      else {
         mPieceUnderCursor = -1;
         wxFrame* parentPtr = (wxFrame*)this->GetParent();
         parentPtr->SetStatusText(wxT(""), 1);
         parentPtr->SetStatusText(wxT(""), 2);
      }
   }
   else if (event.Dragging()) {
      if (mDragState == DRAG_START) {
         // Start dragging if we've moved beyond a few pixels
         int tolerance = 2;

         wxPoint pos = event.GetPosition();
         int dx = abs(pos.x - mStartPosition.x); 
         int dy = abs(pos.y - mStartPosition.y);
         if (dx <= tolerance && dy <= tolerance)
            return;

         // Start the drag proper
         mDragState = DRAG_IN_PROGRESS;

         const wxPen* penPtr;
         const wxBrush* brushPtr;
         int x,y,radius;
         
         if (mPieceInfo[mStartIndex].mSide != mpEngine->GetSide()) {
            penPtr = wxRED_PEN;
            brushPtr = wxRED_BRUSH;
         }
         else {
            penPtr = wxGREEN_PEN;
            brushPtr = wxGREEN_BRUSH;
         }

         // Erase the dragged piece from the canvas image
         wxMemoryDC memDC;
         memDC.SelectObject(*mpCanvasBitmap);
         ErasePiece(memDC, mStartIndex);
         if (mPassMove) {
            int column = mBoardColumns - 1 - (mStartIndex % mBoardColumns);
            int row    = mStartIndex / mBoardColumns;
            x = (2*column + 1) * (mSquareWidth + 1) / 2 + BORDER_WIDTH + 1;
            y = (2*row + 1) * (mSquareHeight + 1) / 2 + BORDER_WIDTH + 1;
            radius = mSquareWidth / 4;
            
            memDC.SetPen(*penPtr);
            memDC.SetBrush(*brushPtr);
            memDC.DrawCircle(x, y, radius);
         }
         memDC.SelectObject(wxNullBitmap);
         
         wxClientDC dc(this);
         ErasePiece(dc, mStartIndex);
         if (mPassMove) {
            dc.SetPen(*penPtr);
            dc.SetBrush(*brushPtr);
            dc.DrawCircle(x, y, radius);
         }

         // Determine what kind of piece the user has picked up
         int index;
         if (mPriorIndex == -1) {
            index = mStartIndex;
         }
         else {
            index = mPriorIndex;
         }
         int piecetype = mPieceInfo[index].mType;
         int imageindex;
         if (mPieceInfo[index].mPromoted) {
            imageindex = mImageMap[PROMOTED][piecetype];
         }
         else {
            imageindex = mImageMap[UNPROMOTED][piecetype];
         }
         bool rotated = (mPieceInfo[index].mSide == WHITE);
         if (rotated) {
            imageindex = (mImageColumns * mImageRows) - 1 - imageindex;
         }
                
         // Create a fresh image of our piece
         wxMemoryDC srcDC;
         wxBitmap* srcBitmapPtr = rotated ? mpPiecesBitmap[ROTATED]
                                          : mpPiecesBitmap[NORMAL];
         wxBitmap tempBitmap(mSquareWidth, mSquareHeight);
         
         srcDC.SelectObject(*srcBitmapPtr);
         memDC.SelectObject(tempBitmap);
         memDC.Blit(0, 0, mSquareWidth, mSquareHeight, &srcDC,
                    (imageindex % mImageColumns) * mSquareWidth,
                    (imageindex / mImageColumns) * mSquareHeight);
         memDC.SelectObject(wxNullBitmap);
         srcDC.SelectObject(wxNullBitmap);
        
         // Create a mask for the piece's image
         wxMask* mask = new wxMask(tempBitmap, 
                                   mShogiApp::PiecesBackgroundColour());
         tempBitmap.SetMask(mask);

         // Create a drag image using the piece image and a cursor
         mpDragImage = new wxDragImage(tempBitmap, *mpClosedHandCursor);
         
         // Calculate the offset between the top-left corner of
         // the drag image and its new position
         wxPoint hotspot = mStartPosition - GetUpperLeftCorner(mStartIndex);

         // Start the actual drag
         if (!mpDragImage->BeginDrag(hotspot, this, false)) {
            delete mpDragImage;
            mpDragImage = 0;
            mDragState = DRAG_NONE;
         }
         else {
            mpDragImage->Move(event.GetPosition());
            mpDragImage->Show();
         }
      }
      else if (mDragState == DRAG_IN_PROGRESS) {
         // Continue dragging the piece
         mpDragImage->Move(event.GetPosition());
      }
   }
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  OnMouseLeftUp
// Description:  Responds to releasing the left mouse button
//------------------------------------------------------------------------
void BoardCanvas::OnMouseLeftUp(wxMouseEvent& event)
{
   if (mDragState == DRAG_START || mDragState == DRAG_ILLEGAL) {
      mDragState = DRAG_NONE;
      mPriorIndex = -1;
      if (!mSquares.empty()) {
         // Prepare the canvas bitmap for drawing
         wxMemoryDC memDC;
         memDC.SelectObject(*mpCanvasBitmap);
         
         // Redraw the board and pieces
         memDC.DrawBitmap(*mpBoardBitmap, 0, 0);
         DrawPieces(memDC);
      
         // De-select the canvas bitmap from the memory device context
         memDC.SelectObject(wxNullBitmap);

         // Redisplay the canvas on the screen
         wxClientDC dc(this);
         dc.DrawBitmap(*mpCanvasBitmap, 0, 0);
      }
   }
   else if (mDragState == DRAG_IN_PROGRESS) {
      mDragState = DRAG_NONE;
      
      // Prepare the canvas bitmap for drawing
      wxMemoryDC memDC;
      memDC.SelectObject(*mpCanvasBitmap);

      // Calculate the upper left corner of the starting square
      wxPoint start = GetUpperLeftCorner(mStartIndex);

      // Calculate the upper left corner of the dragged image
      wxPoint finish = start + event.GetPosition() - mStartPosition;
      
      // Calculate the coordinates of the middle of the dragged image
      wxPoint middle;
      middle.x = (2*finish.x + mSquareWidth) / 2;
      middle.y = (2*finish.y + mSquareHeight) / 2;
   
      // Find the board index of the square that the middle
      // of the dragged image lies inside
      int finishIndex = FindSquare(middle);

      bool moveSucceeded = false;
      
      // Make sure we're on the board proper
      if (finishIndex > -1) {
         // Is this possibly the start of a two part move?
         if (mpEngine->CanMoveTwice(mStartIndex, finishIndex)) {
            mPriorIndex = mStartIndex;
            mDisplayIndex = finishIndex;
            moveSucceeded = true;
         }
         else {
            // Construct a move string from the start 
            // and finish square indices
            string move;
            if (mPriorIndex == -1) {
               move = CreateMoveString(mStartIndex, finishIndex);
            }
            else {
               move = CreateMoveString(mPriorIndex, 
                                       mStartIndex, 
                                       finishIndex);
               mPriorIndex = -1;
            }

            // Can we promote on this move?
            if ( mpEngine->CanPromote(move) ) {
               wxString msg (wxT("Promote this piece?"));
               int answer = wxMessageBox(msg, wxT("Promotion Opportunity"), 
                                         wxYES_NO | wxICON_QUESTION, 
                                         this->GetParent());
               if (answer == wxYES) {
                  // Modify the move string for promotion
                  move += wxT('+');
               }
            }

            // Attempt to make this move
            moveSucceeded = mpEngine->MakeMove(move);
         }
      }

      // Hide the drag image
      mpDragImage->Hide();
      mpDragImage->EndDrag();
      delete mpDragImage;
      mpDragImage = 0;

      // Redraw the board and pieces
      memDC.DrawBitmap(*mpBoardBitmap, 0, 0);
      DrawPieces(memDC);
      
      // De-select the canvas bitmap from the memory device context
      memDC.SelectObject(wxNullBitmap);

      // Redisplay the canvas on the screen
      wxClientDC dc(this);
      dc.DrawBitmap(*mpCanvasBitmap, 0, 0);
      
      // Did the move succeed?
      if ( moveSucceeded ) {
         int side = mpEngine->GetSide();
         // Is the game over?
         if ( mpEngine->IsGameFinished() ) {
            wxString msg( mpEngine->GameStatusMessage().c_str(), wxConvLibc );
            wxMessageBox(msg, wxT("Game Over"), wxOK |wxICON_INFORMATION,
                         this->GetParent());
         }
         // Is it now the computer's turn?
         else if (mPlayerType[side] == COMPUTER) {
            EngineThread* search_thread = new EngineThread(this, mpEngine);
            if ( search_thread->Create() != wxTHREAD_NO_ERROR ) {
               wxString msg( wxT("Unable to create a new thread.") );
               wxMessageBox(msg, wxT("Thread Error"), wxOK |wxICON_INFORMATION,
                            this->GetParent());
            }
            search_thread->Run();
         }
      }
      else {
         wxString msg( wxT("This piece cannot make that move.") );
         wxMessageBox(msg, wxT("Invalid move"), wxOK |wxICON_INFORMATION,
                      this->GetParent());
      }
   }
}

//------------------------------------------------------------------------
//       Class:  BoardCanvas
//      Method:  OnEngineEvent
// Description:  Responds to the engine's attempting to make a move
//------------------------------------------------------------------------
void BoardCanvas::OnEngineEvent(wxCommandEvent& event)
{
   bool succeeded = (bool)event.GetInt();

   // Did the engine succeed in making a move?
   if (succeeded) {
      // Prepare the canvas bitmap for drawing
      wxMemoryDC memDC;
      memDC.SelectObject(*mpCanvasBitmap);

      // Redraw the board and pieces
      memDC.DrawBitmap(*mpBoardBitmap, 0, 0);
      DrawPieces(memDC);
      
      // De-select the canvas bitmap from the memory device context
      memDC.SelectObject(wxNullBitmap);

      // Redisplay the canvas on the screen
      wxClientDC dc(this);
      dc.DrawBitmap(*mpCanvasBitmap, 0, 0);
      
      int side = mpEngine->GetSide();
      // Is the game over?
      if ( mpEngine->IsGameFinished() ) {
         wxString msg( mpEngine->GameStatusMessage().c_str(), wxConvLibc );
         wxMessageBox(msg, wxT("Game Over"), wxOK |wxICON_INFORMATION,
                      this->GetParent());
      }
      // Is it now the computer's turn again?
      else if (mPlayerType[side] == COMPUTER) {
         EngineThread* search_thread = new EngineThread(this, mpEngine);
         if ( search_thread->Create() != wxTHREAD_NO_ERROR ) {
               wxString msg( wxT("Unable to create a new thread.") );
               wxMessageBox(msg, wxT("Thread Error"), wxOK |wxICON_INFORMATION,
                            this->GetParent());
         }
         search_thread->Run();
      }
   }
   else {
      wxString msg( wxT("Unable to find a move.") );
      wxMessageBox(msg, wxT("No Move"), wxOK |wxICON_INFORMATION,
                   this->GetParent());
   }
}

