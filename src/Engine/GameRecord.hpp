////////////////////////////////////////////////////////////////////////////////
// Name:         GameRecord
// Description:  Interface for a class that represents a game record
// Created:      Apr 20, 2004 4:03:50 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef GAMERECORD_H
#define GAMERECORD_H

// STL headers
#include <string>
#include <vector>

// Forward declarations
class Notator;
class Move;
class Game;

class GameRecord
{
public:
   // ==========================================================================
	// Constructors and Destructors
	// ==========================================================================
	GameRecord(Game* game);

   // ==========================================================================
	// Operations
	// ==========================================================================
   void SetGame(Game* game);

   void AddMove(Move* move);
   Move* PopMove();
   Move* LastMove();
   
   bool IsEmpty() { return mMoveList.empty(); }
   
   std::string LastEntry();
   std::string History();
   std::string Result();
   
   void Save(const std::string& filename, const std::string& gamename);
   void Reset();
   
private:
   // ==========================================================================
	// Data
	// ==========================================================================
   Game*    mpGame;            // Pointer to the current game object
   Notator* mpNotator;         // Object that reads and writes move notation
   
   std::vector<Move> mMoveList; // List of moves in the game
   std::vector<std::string> mMoveNotations; // List of move notations in game
};

#endif // GAMERECORD_H
