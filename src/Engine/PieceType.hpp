////////////////////////////////////////////////////////////////////////////////
// Name:         PieceType.hpp
// Description:  Interface for an abstract class that represents a piece type
// Created:      May 1, 2004 12:17:49 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef PIECETYPE_HPP
#define PIECETYPE_HPP

// STL headers
#include <vector>
#include <string>

// Forward declarations
class Move;
class Piece;

class PieceType
{
public:
   virtual ~PieceType() {};
   
   // =========================================================================
   // Operations
   // =========================================================================
   virtual bool ThreatensSquare(Piece* piece, int square) = 0;
   virtual void GenerateCaptures(std::vector<Move>& movelist, Piece* piece) = 0;
   virtual void GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece) = 0;

   // =========================================================================
   // Data
   // =========================================================================
   
   int          mValue;       // Value assigned to this piece type
   int          mTypeValue;   // Value that identifies this piece type
   std::string  mNotation;    // Notation string for this piece type
   std::string  mNames[2];    // English and Japanese names for this piece type
   std::string  mDescription; // Description of this piece type's move
   
   std::vector<int> mWeights[2]; // Weights for evaluating the worth of pieces
                                 // of this piece type
};

#endif // PIECETYPE_HPP
