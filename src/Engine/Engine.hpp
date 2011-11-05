////////////////////////////////////////////////////////////////////////////////
// Name:         Engine.hpp
// Description:  Interface for class that represents the core mShogi
//               shogi variants playing engine
// Created:      Apr 20, 2004 9:49:57 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef ENGINE_HPP
#define ENGINE_HPP

// Standard C++ header files
#include <iostream>

// STL header files
#include <vector>
#include <string>

// Local header file includes
#include "PieceInfo.hpp"
#include "ObjectFactory.hpp"

// Forward declarations
class Game;
class GameRecord;
class Search;

class Engine
{
public:
   // ======================================================================
   // Type Definitions
   // ======================================================================
   typedef ObjectFactory<std::string, Game> GameFactory;
   typedef ObjectFactory<std::string, Search> SearchFactory;
   
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   Engine();
   ~Engine();

   // ======================================================================
   // Accessors
   // ======================================================================
   bool SetGame(const std::string& name);
   const std::string& GetGame() { return mGameName; }

   bool SetSearch(const std::string& name);
   
   void SetHashSize(int size);
   void SetSearchTime(int time);
   void SetSearchDepth(int depth);
   void SetSearchNodes(int nodes);
   
   void SetQuiescence();
   void SetQuiescence(bool value);
   void SetQuiescenceDepth(int depth);
   
   void SetSide(int side) { mSide = side; }

   void StopSearch() { mStopSearch = true; }
   bool SearchAborted() { return mStopSearch; }

   void StartSearching() { mIsSearching = true; }
   void FinishSearching() { mIsSearching = false; }
   bool IsSearching() const { return mIsSearching; }
   
#ifdef DEBUG
   void SetLog();
   void SetLog(bool value);
#endif
   
   int GetSide() { return mSide; }
   int GetSearchTime();
   
   // ======================================================================
   // Operations
   // ======================================================================
   bool IsValidMove(const std::string& movestring);
   bool CanPromote(const std::string& movestring);
   bool CanMoveTwice(int start, int finish);
   
   bool MakeMove(const std::string& movestring);
   bool MakeBestMove(int side, std::string& movestring);
   bool SuggestMove(int side, std::string& movestring);
   bool UnmakeMove();

   bool IsPieceOnSquare(int square);
   bool IsPieceColor(int square, int side);
   std::string GetPieceName(int piecetype, int language=0, 
                            bool promoted=false);

   bool IsGameFinished();
   std::string GameStatusMessage();

   void PrintSearchStatistics(std::ostream& output);
   void PrintBoard(std::ostream& output);
   void PrintLastMove(std::ostream& output);
   void PrintHistory(std::ostream& output);
   void PrintResult(std::ostream& output);

   void GetPieceInfo(std::vector<PieceInfo>& squares);
   void GetDestinationSquares(int square, std::vector<int>& squares);
   void GetSecondSquares(int start, int middle, std::vector<int>& squares);
   
   void GetGameList(std::vector<std::string>& games);
   void GetSearchList(std::vector<std::string>& searches);

   void RestartGame();
   void SaveGame(const std::string& filename);
   bool LoadGame(const std::string& filename, std::string& message);

private:
   // ======================================================================
   // Data
   // ======================================================================
   GameFactory* mpGameFactory;     // Factory for generating instances of
                                   // of known game classes
   SearchFactory* mpSearchFactory; // Factory for generating instances of
                                   // of known search classes
   
   std::string mGameName;          // Name of the type of game we're playing
   std::string mSearchName;        // Name of the search class that we're using 
   
   Game* mpGame;                   // Object that knows how to play a game      
   Search* mpSearch;               // Searches a game position
   GameRecord* mpGameRecord;       // Record of the current game

   int mSide;                      // Side to move

   bool mIsSearching;              // Flag that indicates whether the
                                   // engine is currently searching
   bool mStopSearch;               // Flag indicating whether the 
                                   // engine needs to stop searching
};


#endif // ENGINE_HPP
