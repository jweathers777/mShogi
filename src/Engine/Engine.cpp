////////////////////////////////////////////////////////////////////////////////
// Name:         Engine.cpp
// Description:  Implementation for class that represents the core mShogi
//               shogi variants playing engine
// Created:      Apr 20, 2004 9:49:57 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// =========================================================================
// Header file includes
// =========================================================================

// Standard C header files

// Standard C++ header files
#include <fstream>

// STL header files

// Local header files
#include "common.hpp"
#include "Engine.hpp"
#include "Games.hpp"
#include "GameRecord.hpp"
#include "Searches.hpp"
#include "Piece.hpp"
#include "Move.hpp"

using std::ostream;
using std::ifstream;
using std::endl;
using std::string;
using std::vector;

//--------------------------------------------------------------------------
//       Class:  Engine
//      Method:  Engine
// Description:  Construct an instance of an mShogi engine
//--------------------------------------------------------------------------
Engine::Engine()
{
   AbstractCreator<Game>* game_creator;
   AbstractCreator<Search>* search_creator;
   
   // Obtain the one and only game factory instance
   mpGameFactory = GameFactory::GetInstance();
   
   // Obtain the one and only search factory instance
   mpSearchFactory = SearchFactory::GetInstance();

   // Register all games that the engine supports
   game_creator = new Creator<Game, MiniShogi>;
   mpGameFactory->Register("minishogi", game_creator);
   
   game_creator = new Creator<Game, ChuShogi>;
   mpGameFactory->Register("chushogi", game_creator);
   
   game_creator = new Creator<Game, TenjikuShogi>;
   mpGameFactory->Register("tenjikushogi", game_creator);
   
   // Register all searches that the engine supports
   search_creator = new Creator<Search, AlphaBetaSearch>;
   mpSearchFactory->Register("alpha-beta", search_creator);
   
   search_creator = new Creator<Search, PVSearch>;
   mpSearchFactory->Register("principle-variation", search_creator);
   
   search_creator = new Creator<Search, AlphaBetaMemory>;
   mpSearchFactory->Register("alpha-beta-memory", search_creator);
   
   search_creator = new Creator<Search, IterDeepSearch<AlphaBetaMemory> >;
   mpSearchFactory->Register("iter-deep", search_creator);
   
   search_creator = new Creator<Search, PVMSearch>;
   mpSearchFactory->Register("principle-var-mem", search_creator);
   
   search_creator = new Creator<Search, IterDeepSearch<PVMSearch> >;
   mpSearchFactory->Register("iter-deep-pvm", search_creator);
   
   // Initialize engine parts
   mGameName = "tenjikushogi";
   mpGame = mpGameFactory->Create(mGameName);

   mSearchName = "iter-deep";
   mpSearch = mpSearchFactory->Create(mSearchName);
   mpSearch->SetEngine(this);
   mpSearch->SetGame(mpGame);
   
   mpGameRecord = new GameRecord(mpGame);

   mSide = 0;
   mStopSearch = false;
}

//--------------------------------------------------------------------------
//       Class:  Engine
//      Method:  ~Engine
// Description:  Destroys this instance of an mShogi engine
//--------------------------------------------------------------------------
Engine::~Engine()
{
   if (mpSearch) {
      delete mpSearch;
      mpSearch = 0;
   }
   
   if (mpGame) {
      delete mpGame;
      mpGame = 0;
   }
   
   if (mpGameRecord) {
      delete mpGameRecord;
      mpGameRecord = 0;
   }

   if (mpGameFactory) {
      delete mpGameFactory;
      mpGameFactory = 0;
   }

   if (mpSearchFactory) {
      delete mpSearchFactory;
      mpSearchFactory = 0;
   }

}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetGame
// Description:  Sets the current game for the engine
//-------------------------------------------------------------------------
bool Engine::SetGame(const string& name)
{
   bool result = false;
   if (mpGameFactory) {
      // Destroy the current game if one exists
      if (mpGame)
         delete mpGame;

      // Create and initialize a new game
      mpGame = mpGameFactory->Create(name);

      if (mpGame) {
         // Update engine parts
         mGameName = name;
         
         mpGameRecord->SetGame(mpGame);
         mpSearch->SetGame(mpGame);
         
         mpGameRecord->Reset();
         mpSearch->Reset();

         mSide = 0;
         mStopSearch = false;

         result = true;
      }
   }

   return result;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetSearch
// Description:  Sets the current search class for the engine
//-------------------------------------------------------------------------
bool Engine::SetSearch(const string& name)
{
   bool result = false;
   if (mpSearchFactory) {
      // Destroy the current search if one exists
      if (mpSearch)
         delete mpSearch;

      // Create and initialize a new search
      mpSearch = mpSearchFactory->Create(name);

     if (mpSearch) {
       // Update the search name
       mSearchName = name;

       // Set a reference to this engine
       mpSearch->SetEngine(this);

       // Associate the current game with this search
       mpSearch->SetGame(mpGame);

       result = true;
     }
   }

   return result;
}
        
//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetHashSize
// Description:  Sets the hash size for transposition tables
//-------------------------------------------------------------------------
void Engine::SetHashSize(int size)
{
   mpSearch->SetHashSize(size);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetSearchTime
// Description:  Set the maximum time spent searching
//-------------------------------------------------------------------------
void Engine::SetSearchTime(int time)
{
   mpSearch->SetSearchTime(time);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetSearchDepth
// Description:  Set the maximum search tree depth
//-------------------------------------------------------------------------
void Engine::SetSearchDepth(int depth)
{
   mpSearch->SetSearchDepth(depth);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetSearchNodes
// Description:  Set the maximum number of nodes searched
//-------------------------------------------------------------------------
void Engine::SetSearchNodes(int nodes)
{
   mpSearch->SetSearchNodes(nodes);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetQuiescence
// Description:  Toggle quiescence search on and off
//-------------------------------------------------------------------------
void Engine::SetQuiescence()
{
   mpSearch->SetQuiescence();
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetQuiescence
// Description:  Toggle quiescence search on and off
//-------------------------------------------------------------------------
void Engine::SetQuiescence(bool value)
{
   mpSearch->SetQuiescence(value);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetQuiescentDepth
// Description:  Set the depth of the quiescence search
//-------------------------------------------------------------------------
void Engine::SetQuiescenceDepth(int value)
{
   mpSearch->SetQuiescenceDepth(value);
}

#ifdef DEBUG
//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetLog
// Description:  Toggle debug log on and off
//-------------------------------------------------------------------------
void Engine::SetLog()
{
   keeplog = !keeplog;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SetLog
// Description:  Set whether the debug log is on
//-------------------------------------------------------------------------
void Engine::SetLog(bool value)
{
   keeplog = value;
}
#endif

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  GetSearchTime
// Description:  Get the maximum time allowed for searching
//-------------------------------------------------------------------------
int Engine::GetSearchTime()
{
   return mpSearch->GetSearchTime();
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  IsValidMove
// Description:  Verifies whether a given string is a valid move
//-------------------------------------------------------------------------
bool Engine::IsValidMove(const string& movestring)
{
   Move* move = mpGame->ParseMove(movestring);
   bool result = false;
   
   if (move) {
      result = true;
      
      delete move;
      move = 0;
   }

   return result;
}

//------------------------------------------------------------------------
//       Class:  Engine
//      Method:  CanPromote
// Description:  Returns true if this move offers the opportunity to promote
//------------------------------------------------------------------------
bool Engine::CanPromote(const string& movestring)
{
   Move* move = mpGame->ParseMove(movestring);
   return ( (move != 0) && mpGame->CanPromote(move) );
}

//------------------------------------------------------------------------
//       Class:  Engine
//      Method:  CanMoveTwice
// Description:  Returns true if this move offers the opportunity to move
//               twice in one move
//------------------------------------------------------------------------
bool Engine::CanMoveTwice(int start, int finish)
{
   return mpGame->CanMoveTwice(start, finish);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  MakeMove
// Description:  Attempt to make the move represented by the given string
//-------------------------------------------------------------------------
bool Engine::MakeMove(const string& movestring)
{
   Move* move = mpGame->ParseMove(movestring);
   bool result = (move != 0) && mpGame->IsLegal(move, mSide);

   if (result) {
      mpGame->MakeMove(move);
      mpGameRecord->AddMove(move);
      mSide = 1 - mSide;
   }

   if (move) {
      delete move;
      move = 0;
   }
      
   return result;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  MakeBestMove
// Description:  Make an optimal move for the given side 
//               in the current position
//-------------------------------------------------------------------------
bool Engine::MakeBestMove(int side, string& movestring)
{
   Move* move = mpSearch->GetBestMove(side);
   bool result = (move != 0);

   if (result) {
      movestring = mpGame->Notate(move);

      mpGame->MakeMove(move);
      mpGameRecord->AddMove(move);
      mSide = 1 - mSide;

      delete move;
      move = 0;
   }

   return result;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SuggestMove
// Description:  Suggest an optimal move for the given side 
//               in the current position
//-------------------------------------------------------------------------
bool Engine::SuggestMove(int side, string& movestring)
{
   Move* move = mpSearch->GetBestMove(side);
   bool result = (move != 0);

   if (result) {
      movestring = mpGame->Notate(move);

      delete move;
      move = 0;
   }
   return result;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  UnmakeMove
// Description:  Unmake the last move made in the current game
//-------------------------------------------------------------------------
bool Engine::UnmakeMove()
{
   bool result = !mpGameRecord->IsEmpty();

   if (result) {
      Move* move = mpGameRecord->PopMove();
      mpGame->UnmakeMove(move);
      mSide = 1 - mSide;

      delete move;
      move = 0;
   }

   return result;
}

//------------------------------------------------------------------------
//       Class:  Engine
//      Method:  IsPieceOnSquare
// Description:  Returns true if there is a piece on the given square 
//------------------------------------------------------------------------
bool Engine::IsPieceOnSquare(int square)
{
   return mpGame->IsPieceOnSquare(square);
}

//------------------------------------------------------------------------
//       Class:  Engine
//      Method:  IsPieceColor
// Description:  Returns true if there is a piece on the given square 
//------------------------------------------------------------------------
bool Engine::IsPieceColor(int square, int side)
{
   return mpGame->IsPieceColor(square, side);
}

//------------------------------------------------------------------------
//       Class:  Engine
//      Method:  GetPieceName
// Description:  Returns the unpromoted or promoted name associated with 
//               the given piece type in the given language
//------------------------------------------------------------------------
string Engine::GetPieceName(int piecetype, int language, bool promoted)
{
   return mpGame->GetPieceName(piecetype, language, promoted);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  IsGameFinished
// Description:  Returns true if the game is finished
//-------------------------------------------------------------------------
bool Engine::IsGameFinished()
{
   return (mpGame->mStatus != Game::IN_PROGRESS);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  GameStatusMessage
// Description:  Returns a string with a message about the game status
//-------------------------------------------------------------------------
string Engine::GameStatusMessage()
{
   string message;
   switch (mpGame->mStatus) {
   case Game::IN_PROGRESS:
      message = "The game is still in progress.";
      break;
   case Game::BLACK_WIN:
      message = "The game is over. Black has won.";
      break;
   case Game::WHITE_WIN:
      message = "The game is over. White has won.";
      break;
   default:
      message = "The game is a draw.";
   }

   return message;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  PrintSearchStatistics
// Description:  Print the search statistics
//-------------------------------------------------------------------------
void Engine::PrintSearchStatistics(ostream& output)
{
   output << mpSearch->GetStatistics() << endl;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  PrintBoard
// Description:  Print the board position to an output stream
//-------------------------------------------------------------------------
void Engine::PrintBoard(ostream& output)
{
   output << mpGame->NotateBoard() << endl;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  PrintLastMove
// Description:  Print the last move made in the current game to 
//               a given output stream
//-------------------------------------------------------------------------
void Engine::PrintLastMove(ostream& output)
{
   output << mpGameRecord->LastEntry() << endl;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  PrintHistory
// Description:  Print the current game's history to an output stream
//-------------------------------------------------------------------------
void Engine::PrintHistory(ostream& output)
{
   output << mpGameRecord->History() << endl;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  PrintResult
// Description:  Print the current game's result if the game if is finished
//-------------------------------------------------------------------------
void Engine::PrintResult(ostream& output)
{
   if ( IsGameFinished() ) {
      output << mpGameRecord->Result() << endl;
   }
}

//------------------------------------------------------------------------
//       Class:  Engine
//      Method:  GetPieceInfo
// Description:  Return a vector of PieceInfo structures that represent
//               the squares on the current board
//------------------------------------------------------------------------
void Engine::GetPieceInfo(vector<PieceInfo>& squares)
{
   mpGame->GetPieceInfo(squares);
}

//------------------------------------------------------------------------
//       Class:  Engine
//      Method:  GetDestinationSquares
// Description:  Return a vector of integers representing possible
//               destination squares for the piece at the given 
//               board location
//------------------------------------------------------------------------
void Engine::GetDestinationSquares(int index, vector<int>& squares)
{
   mpGame->GetDestinationSquares(index, squares);
}

//------------------------------------------------------------------------
//       Class:  Engine
//      Method:  GetSecondSquares
// Description:  Return a vector of integers representing possible
//               destination squares for the piece at the given 
//               board location when a two-part move
//------------------------------------------------------------------------
void Engine::GetSecondSquares(int start, int middle, vector<int>& squares)
{
   mpGame->GetSecondSquares(start, middle, squares);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  GetGameList
// Description:  Return a list of the names of the games the engine supports
//-------------------------------------------------------------------------
void Engine::GetGameList(vector<string>& games)
{
   mpGameFactory->GetKeys(games);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  GetSearchList
// Description:  Return a list of the names of the search types
//               supported by the engine
//-------------------------------------------------------------------------
void Engine::GetSearchList(vector<string>& searches)
{
   mpSearchFactory->GetKeys(searches);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  RestartGame
// Description:  Restart the current game
//-------------------------------------------------------------------------
void Engine::RestartGame()
{
   mpGame->Reset();
   mpGameRecord->Reset();
   mpSearch->Reset();

   mSide = 0;
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  SaveGame
// Description:  Save the current game to a file
//-------------------------------------------------------------------------
void Engine::SaveGame(const string& filename)
{
   mpGameRecord->Save(filename, mGameName);
}

//-------------------------------------------------------------------------
//       Class:  Engine
//      Method:  LoadGame
// Description:  Load a saved game from a file
//-------------------------------------------------------------------------
bool Engine::LoadGame(const string& filename, string& message)
{
   ifstream input(filename.c_str());

   // Make sure the file exists and that it is opened correctly
   if ( input.is_open() ) {
      string line;
      
      getline(input, line);
      
      // Attempt to set the game type
      if ( SetGame(line) ) {
         int number;
         char c;
         string name;
         bool move = false;
         
         while (!input.eof()) {
            // Move number
            input >> number;
            if (input.eof()) return true;

            if (!input) {
               message = "Invalid move number.";
               return false;
            }

            // Period after move number
            input >> c;
            if (!input) {
               message = "Unknown error processing move.";
               return false;
            }
            else if (c != '.') {
               message = "Invalid move format.";
               return false;
            }

            // Piece name
            name = "";
            move = false;
            while (!move) {
               // Get the next character
               input >> c;
               if (!input) {
                  message = "Invalid piece name.";
                  return false;
               }
               
               // Look for first digit to indicate
               // the finish of the piece name
               if (c < '0' || c > '9') {
                  name += c;
               }
               else {
                  input.unget();
                  move = true;
               }
            }

            // Retrieve rest of the line
            getline(input, line);

            // Attempt to parse it as a move
            if ( !IsValidMove(line) ) {
               message = line + " is not a valid move.";
               return false;
            }

            // Can we actually make this move?
            if ( !MakeMove(line) ) {
               message = line + " is not a legal move.";
               return false;
            }
         }
      }
      else {
         message = "Invalid game type.";
         return false;
            
      }
   }
   else {
      message = "Unknown error opening file.";
      return false;
   }
   
   return true;
}

