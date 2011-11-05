////////////////////////////////////////////////////////////////////////////////
// Name:         Evaluator.hpp
// Description:  Interface for a class that evaluates positions in a game
// Created:      Apr 26, 2004 8:51:06 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef EVALUATOR_H
#define EVALUATOR_H

// STL declarations
#include <vector>

// Forward declarations
class Piece;

class Evaluator
{
public:
   // ==========================================================================
   // Construction and Destruction
   // ==========================================================================
	Evaluator(std::vector<Piece>* pieces);
   ~Evaluator();
   
   // ==========================================================================
   // Operations
   // ==========================================================================
   int FullEvaluation(int side);
   
private:   
   // ==========================================================================
   // Data
   // ==========================================================================
   std::vector<Piece>* mpPieces;        // Pieces in the game
};

#endif // EVALUATOR_H
