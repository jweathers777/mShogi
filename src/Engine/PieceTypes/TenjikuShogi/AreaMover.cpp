////////////////////////////////////////////////////////////////////////////
// Name:         AreaMover.cpp
// Description:  A class that represents the area-move capabilities 
//               of a piece that can make 3-step area moves
// Created:      08/29/2004 03:37:47 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// Local header files
#include "AreaMover.hpp"
#include "Board.hpp"

using std::vector;

// Static data initialization
AreaMover* AreaMover::pInstance = 0;

//--------------------------------------------------------------------------
//       Class:  AreaMover
//      Method:  AreaMover
// Description:  Construct an instance of a AreaMover object
//--------------------------------------------------------------------------
AreaMover::AreaMover(Board* board)
{
   mpBoard = board;
   mWidth = board->GetWidth();
   mHeight = board->GetHeight();
   mSize = mWidth*mHeight;

   // Initialize attack bitboards
   InitAttackBitboards();

   // Initialize the area paths
   InitAreaPaths();   
}

//--------------------------------------------------------------------------
//       Class:  AreaMover
//      Method:  GetInstance
// Description:  Returns a pointer to the one instance of the AreaMover class
//--------------------------------------------------------------------------
AreaMover* AreaMover::GetInstance(Board* board)
{
   if (pInstance) {
      if (pInstance->mpBoard == board) { 
         return pInstance;
      }
      else {
         delete pInstance;
      }
   }

   pInstance = new AreaMover(board);

   return pInstance;
}

//--------------------------------------------------------------------------
//       Class:  AreaMover
//      Method:  ~AreaMover
// Description:  Destroys this instance of a AreaMover
//--------------------------------------------------------------------------
AreaMover::~AreaMover()
{
   int i;
   if (pInstance) {
      delete [] mAttackBitboards;
      for (i = 0; i < mSize; i++) {
         delete [] mAreaPaths[i];
      }
      delete [] mAreaPaths;
   }
}

//--------------------------------------------------------------------------
//       Class:  AreaMover
//      Method:  HasLegalPath
// Description:  Returns true if there exists a legal path from 
//               the start square to the finish square
//--------------------------------------------------------------------------
bool AreaMover::HasLegalPath(int start, int finish)
{
   unsigned int i;
   int delta = abs(start - finish);
   bool pathfound = false;
   Bitboard result(mSize), zero(mSize);
   vector<Bitboard>* paths;
   
   if ( (delta == 1)      || (delta == (mWidth-1)) ||
        (delta == mWidth) || (delta == (mWidth + 1)) ) {
      // This is a one step path which is always legal
      pathfound = true;
   }
   else {
      // This is a two or three step path and we need to verify
      // that it isn't blocked by other pieces
      paths = &(mAreaPaths[start][finish]);
      for (i = 0; i < paths->size(); i++) {
         result = (*paths)[i] & mpBoard->mAllPieces;
         if (result == zero) {
            pathfound = true;
            break;
         }
      }
   }

   return pathfound;
}

//--------------------------------------------------------------------------
//       Class:  AreaMover
//      Method:  InitAttackBitboards
// Description:  Initialize the attack bitboard patterns for area moves
//--------------------------------------------------------------------------
void AreaMover::InitAttackBitboards()
{
   Bitboard row(mSize);
   Bitboard base(mSize);
   int i, start;
   
   // Define the center of a 7x7 area when its upper right 
   // corner is the upper right corner of our board
   int center = 3*mWidth + 3;
   
   // Create a new array of bitboards
   mAttackBitboards = new Bitboard [mSize];

   // Initialize each bitboard to match the size of the board
   for (i = 0; i < mSize; i++) {
      mAttackBitboards[i].resize(mSize);
   }

   // Fill a row of length 7 with 1's
   for (i = 0; i < 7; i++) {
      row.set(i);
   }

   // Create a 7x7 area centered at "center" filled 
   // with 1's except at the center
   base = row;
   for (i = 1; i < 7; i++) {
      row <<= mWidth;
      base |= row;
   }

   // For each square on the board calculate the
   // associated attack bitboard pattern by recentering
   // the basic pattern that was formed above so that
   // the center is the current start square
   for (start = 0; start < mSize; start++) {
      mAttackBitboards[start] = base;
      ShiftCenter(mAttackBitboards[start], center, start);
      mAttackBitboards[start].set(start, false);
   }
}

//--------------------------------------------------------------------------
//       Class:  AreaMover
//      Method:  InitAreaPaths
// Description:  Initialize the legal path bitboards for area moves
//--------------------------------------------------------------------------
void AreaMover::InitAreaPaths()
{
   int stepone, steptwo, stepthree;
   int square;
   Bitboard bitboard(mSize);

   // Create a new double array of bitboard vectors
   mAreaPaths = new vector<Bitboard>* [mSize];
   for (square = 0; square < mSize; square++) {
      mAreaPaths[square] = new vector<Bitboard> [mSize];
   }

   for (square = 0; square < mWidth*mWidth; square++)
   {
      // Setup bitboard for Path #1
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #2
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #3
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #4
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #5
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #6
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #7
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #8
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #9
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #10
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #11
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #12
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #13
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #14
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #15
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #16
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #17
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #18
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #19
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #20
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #21
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #22
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #23
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #24
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #25
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #26
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #27
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #28
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #29
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #30
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #31
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #32
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            bitboard.set(stepone);
            mAreaPaths[square][steptwo].push_back(bitboard);
            bitboard.set(stepone, false);
         }
      }


      // Setup bitboard for Path #33
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #34
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #35
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #36
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #37
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #38
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #39
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #40
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #41
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #42
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #43
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #44
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #45
      stepone = mpBoard->North(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #46
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #47
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #48
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #49
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #50
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #51
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #52
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #53
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #54
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #55
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #56
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #57
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #58
      stepone = mpBoard->South(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #59
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #60
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #61
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #62
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #63
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #64
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #65
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #66
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #67
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #68
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #69
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #70
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #71
      stepone = mpBoard->East(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #72
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #73
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #74
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #75
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #76
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #77
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #78
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #79
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #80
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #81
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #82
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #83
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #84
      stepone = mpBoard->West(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #85
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #86
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #87
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #88
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #89
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #90
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #91
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #92
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #93
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #94
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #95
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #96
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #97
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #98
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #99
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #100
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #101
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #102
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #103
      stepone = mpBoard->NorthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #104
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #105
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #106
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->North(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #107
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #108
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #109
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #110
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #111
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #112
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #113
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #114
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #115
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #116
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #117
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #118
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #119
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #120
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #121
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #122
      stepone = mpBoard->NorthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #123
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #124
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #125
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #126
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #127
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #128
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->East(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #129
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #130
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #131
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #132
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #133
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #134
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #135
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #136
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #137
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #138
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #139
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #140
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #141
      stepone = mpBoard->SouthEast(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #142
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #143
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #144
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->South(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #145
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #146
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #147
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->West(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #148
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->North(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #149
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #150
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #151
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->NorthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #152
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #153
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->East(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #154
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #155
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthEast(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #156
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->South(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #157
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->West(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #158
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->NorthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #159
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthEast(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }

      // Setup bitboard for Path #160
      stepone = mpBoard->SouthWest(0, square);
      if (stepone != -1) {

         steptwo = mpBoard->SouthWest(0, stepone);
         if (steptwo != -1) {

            stepthree = mpBoard->SouthWest(0, steptwo);
            if (stepthree != -1) {

               bitboard.set(stepone);
               bitboard.set(steptwo);
               mAreaPaths[square][stepthree].push_back(bitboard);
               bitboard.set(stepone, false);
               bitboard.set(steptwo, false);
            }
         }
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  AreaMover
//      Method:  ShiftCenter
// Description:  Change a given bitboard pattern by shifting the center
//               of the defined area from the old center to a new center
//--------------------------------------------------------------------------
void AreaMover::ShiftCenter(Bitboard& bitboard, int oldcenter, int newcenter)
{
   // Nothing to do if the centers are equal
   if (oldcenter == newcenter) return;
   
   int i;

   int x1 = newcenter % mWidth;
   int y1 = newcenter / mWidth;
   int x0 = oldcenter % mWidth;
   int y0 = oldcenter / mWidth;

   int deltaX = x0 - x1;
   int deltaY = y0 - y1;

   // Shift the bitboard pattern from left to right
   if (deltaX < 0) {
      deltaX *= -1; // Change value to positive

      // Shift the pattern deltaX squares 1 square at a time
      while (deltaX > 0) {
         bitboard <<= 1; // Shift pattern one step
         
         // Unset the bits that have wrapped around
         for (i = 0; i < mHeight; i++) {
            bitboard.set(mWidth*i, false);
         }
         deltaX--;
      }
   }
   // Shift the bitboard pattern from right to left
   else if (deltaX > 0) {
      // Shift the pattern deltaX squares 1 square at a time
      while (deltaX > 0) {
         bitboard >>= 1; // Shift pattern one step

         // Unset the bits that have wrapped around
         for (i = 1; i <= mHeight; i++) {
            bitboard.set(mWidth*i - 1, false);
         }
         deltaX--;
      }
   }

   // Shift the bitboard pattern from top to bottom
   if (deltaY < 0) {
      deltaY *= -1; // Change value to positive

      // Shift the pattern deltaY squares at once
      bitboard <<= mWidth*deltaY;
   }
   // Shift the bitboard pattern from bottom to top
   else if (deltaY > 0) {
      // Shift the pattern deltaY squares at once
      bitboard >>= mWidth*deltaY;
   }
}

