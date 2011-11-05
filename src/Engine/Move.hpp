////////////////////////////////////////////////////////////////////////////////
// Name:         Move.hpp
// Description:  Interface for a class that represents a move
// Created:      Apr 20, 2004 1:38:12 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef MOVE_HPP
#define MOVE_HPP

// STL header files
#include <vector>

// Forward declarations
class Piece;
struct MoveRec;

class Move
{
public:
   // ==========================================================================
   // Constructors and Destructors
   // ==========================================================================
   Move(Piece* mover, int source, int destination, Piece* capture=0,
        bool validate=false);
   Move(Piece* mover, int source, int destination, 
        std::vector<Piece*>& captures, bool validate=false);
   Move(Piece* mover, int source, int visited, int destination, 
        Piece* capture=0, bool validate=false);
   Move(Piece* mover, int source, int visited, int destination, 
        std::vector<Piece*>& captures, bool validate=false);
	Move(const Move& move);

   void Copy(const Move& move);
   
   // ==========================================================================
   // Operator
   // ==========================================================================
   bool operator== (const Move& move) const;
   bool operator== (const MoveRec& moverec) const;
   
   // ==========================================================================
   // Operations
   // ==========================================================================
   void FillMoveRec(MoveRec& moverec);
  
   // ==========================================================================
   // Data
   // ==========================================================================
   int mSource;      // Square on which the mover starts
   int mVisited;     // Intermedate square for lion-like moves
   int mDestination; // Square on which the move finishes

   int mValue;       // Material value of this move
   
   Piece* mpMover;   // Pointer to the piece that moves
   std::vector<Piece*> mCaptures; // Vector of pointers to captured pieces
   
   bool mValidate;   // True if custom validation is required for this move
   bool mPromote;    // True if this move promotes the moving piece
   bool mSuicide;    // True if this move results in self-capture
};

#endif // MOVE_HPP
