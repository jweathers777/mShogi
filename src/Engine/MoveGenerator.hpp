////////////////////////////////////////////////////////////////////////////////
// Name:         MoveGenerator.hpp
// Description:  Interface for class that generates moves for a board position
// Created:      Apr 20, 2004 11:43:23 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef MOVEGENERATOR_HPP
#define MOVEGENERATOR_HPP

// STL header files
#include <vector>

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "common.hpp"

// Forward declarations
class Game;
class Move;
class Piece;

class MoveGenerator
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   MoveGenerator(Game* game, int piececount);
   ~MoveGenerator();

   // ======================================================================
   // Operations
   // ====================================================================== 
   void GenerateMoves(std::vector<Move>& movelist, int side);
   
   Move* GetNextMove(std::vector<Move>& movelist, int& phase, int side,
                     Move* hashmove=0);
   Move* GetNextCapture(std::vector<Move>& movelist, int& phase, int side);
   
   void CheckForPromotions(std::vector<Move>& movelist, int side);   
   void CalculateSortValues(std::vector<Move>& movelist);   

   // ======================================================================
   // Constants
   // ======================================================================
   enum {
      PROCESS_HASH_MOVE = 0,
      GENERATE_CAPTURES,
      RETURN_CAPTURES,
      GENERATE_DROPS,
      RETURN_DROPS,
      GENERATE_NONCAPTURES,
      RETURN_NONCAPTURES
   };
      
private:
   // ======================================================================
   // Data
   // ======================================================================
   int mPieceCount;  // Number of pieces per side

   Game* mpGame;     // Pointer to the game that we are playing
   
   std::vector<Piece>* mpPieces;  // Pointers to the pieces in the game
   Bitboard*           mpPromotionZones; // Pointer to Promotion Zone array
};

#endif // MOVEGENERATOR_HPP
