////////////////////////////////////////////////////////////////////////////////
// Name:         ConsoleShell.cpp
// Description:  Implementation for class that represents a console shell 
//               interface for the mShogi shogi variants engine
// Created:      Apr 19, 2004 10:41:50 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// =========================================================================
// Header file includes
// =========================================================================

// Standard C++ header files
#include <iostream>
#include <cctype>
#include <fstream>
#include <sstream>
#include <iomanip>

// STL header files
#include <algorithm>

// Local header files
#include "common.hpp"
#include "ConsoleShell.hpp"
#include "Engine.hpp"
#include "tokenize.hpp"

using namespace std;

//--------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  ConsoleShell
// Description:  Construct an instance of a ConsoleShell object
//--------------------------------------------------------------------------
ConsoleShell::ConsoleShell()
{
   // Initialize the mapping between commands and input handlers
   // and the mapping between commands and help strings
   CreateMaps();

   // Initialize the input stream pointer stack with a pointer to STDIN
   mInputStack.push_back(&cin);

   // Initialize the game engine
   mpEngine = new Engine;
}

//--------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  ~ConsoleShell
// Description:  Destroys this instance of a ConsoleShell
//--------------------------------------------------------------------------
ConsoleShell::~ConsoleShell()
{
   if (mpEngine) {
      delete mpEngine;
      mpEngine = 0;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  StartShell
// Description:  Performs any necessary initialization and then 
//               starts the game console shell by commencing the 
//               main input processing loop
//-------------------------------------------------------------------------
void ConsoleShell::StartShell()
{
   // Print a greeting message
   cout << "Welcome to mShogi Console version " << VERSION << endl;
   cout << "Please report all bugs to the author at: "
        << "<hotanguish@hotmail.com>" << endl;
   cout << "(Type 'help' for a list of available commands)" << endl;

   // The main control loop that parses input and hands the data 
   // over to the appropriate handlers
   mFinished = false;
   MainLoop();
}


//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  MainLoop
// Description:  Main control loop that parses input and passes the data
//               to appropriate input handler methods
//-------------------------------------------------------------------------
void ConsoleShell::MainLoop()
{
   string line;
   vector<string> tokens;
   istream* streamptr;
   
   while (!mFinished) {
      // Prompt the user for input and receive it
      streamptr = mInputStack.back();

      // If this is the top input level, then print
      // a command prompt before accepting new input
      if (streamptr == &cin) {
         cout << endl << "? ";
      }
      getline(*streamptr, line);

      // Convert the input to lower case
      transform(line.begin(), line.end(),
                line.begin(), (int(*)(int))tolower);

      // Tokenize the input
      tokenize(line, tokens, " =");

      // Make sure we actually have input
      if (tokens.size() == 0) {
         cout << "No command given." << endl;
      }
      else {
         // Look up the first token of input to find
         // the appropriate input handler method
         InputHandlerMap::iterator itr = mInputHandlers.find(tokens[0]);

         if (itr == mInputHandlers.end()) {
            // No exact match was found;
            // Let's try for a substring match
            itr = find_if(mInputHandlers.begin(), 
                          mInputHandlers.end(),
                          substr(tokens[0]));
         }
         
         if (itr != mInputHandlers.end()) {
            // Call the handler that we've found
            InputHandlerPtr handlerptr = (*itr).second;
            (*this.*handlerptr)(tokens);
         }
         // Test whether the input line is a valid move
         else if ( mpEngine->IsValidMove(line) ) {
            // Is the game over?
            if ( mpEngine->IsGameFinished() ) {
               cout << mpEngine->GameStatusMessage() << endl;
            }
            // Try to make given move
            else if ( mpEngine->MakeMove(line) ) {
               if (streamptr == &cin) {
                  mpEngine->PrintBoard(cout);
               }
               // Is the game now over?
               if ( mpEngine->IsGameFinished() ) {
                  cout << mpEngine->GameStatusMessage() << endl;
               }
            }
            else {
               // The first token is not a legal move
               cout << tokens[0] << " is not a legal move." << endl;
            }
         }
         else{
            // The first token is not a valid command
            cout << tokens[0] << " is not a valid command." << endl;
         }
      }
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  CreateMaps
// Description:  Map command strings to input handler methods and 
//               command strings to help strings
//-------------------------------------------------------------------------
void ConsoleShell::CreateMaps()
{
   // Map commands strings to command methods
   mInputHandlers.insert(make_pair("count", &ConsoleShell::Count));
   mInputHandlers.insert(make_pair("depth", &ConsoleShell::Depth));
   mInputHandlers.insert(make_pair("display", &ConsoleShell::Display));
   mInputHandlers.insert(make_pair("exit", &ConsoleShell::Exit));
   mInputHandlers.insert(make_pair("games", &ConsoleShell::Games));
   mInputHandlers.insert(make_pair("help", &ConsoleShell::Help));
   mInputHandlers.insert(make_pair("history", &ConsoleShell::History));
   mInputHandlers.insert(make_pair("input", &ConsoleShell::Input));
#ifdef DEBUG
   mInputHandlers.insert(make_pair("log", &ConsoleShell::Log));
#endif
   mInputHandlers.insert(make_pair("new", &ConsoleShell::New));
   mInputHandlers.insert(make_pair("quiescence", &ConsoleShell::Quiescence));
   mInputHandlers.insert(make_pair("qdepth", &ConsoleShell::QuiescenceDepth));
   mInputHandlers.insert(make_pair("quit", &ConsoleShell::Quit));
   mInputHandlers.insert(make_pair("savegame", &ConsoleShell::SaveGame));
   mInputHandlers.insert(make_pair("search", &ConsoleShell::Search));
   mInputHandlers.insert(make_pair("searches", &ConsoleShell::Searches));
   mInputHandlers.insert(make_pair("sethashsize", &ConsoleShell::SetHashSize));
   mInputHandlers.insert(make_pair("setsearch", &ConsoleShell::SetSearch));
   mInputHandlers.insert(make_pair("selfplay", &ConsoleShell::SelfPlay));
   mInputHandlers.insert(make_pair("undo", &ConsoleShell::Undo));
   
   // Map commands strings to help strings
   mHelpStrings.insert(
      make_pair("count", "count N <command>.........execute <command> N times.")
      );
   mHelpStrings.insert(
      make_pair("depth", "depth N...................set search depth to N.")
      );
   mHelpStrings.insert(
      make_pair("display", "display...................display the board.")
      );
   mHelpStrings.insert(
      make_pair("exit", "exit......................restores STDIN to the previous input stream.")
      );
   mHelpStrings.insert(
      make_pair("games", "games.....................list the available game types.")
      );
   mHelpStrings.insert(
      make_pair("help", "help [command]............displays help.")
      );
   mHelpStrings.insert(
      make_pair("history", "history...................displays game history.")
      );
   mHelpStrings.insert(
      make_pair("input","input <filename>..........sets STDIN to <filename>.")
      );
#ifdef DEBUG
   mHelpStrings.insert(
      make_pair("log",  "log on|off................toggle the log on/off.")
      );
#endif
   mHelpStrings.insert(
      make_pair("new", "new [game]................initialize and start a new game.")
      );
   mHelpStrings.insert(
      make_pair("quiescence", "quiescence on|off.........toggle quiescence search on/off.")
      );
   mHelpStrings.insert(
      make_pair("qdepth", "qdepth N..................set quiescence search depth to N.")
      );
   mHelpStrings.insert(
      make_pair("quit", "quit......................quits mShogi.")
      );
   mHelpStrings.insert(
      make_pair("savegame", "savegame <filename>.......saves game to a file.")
      );
   mHelpStrings.insert(
      make_pair("search", "search....................search current for best move.")
      );
   mHelpStrings.insert(
      make_pair("searches", "searches................list the available search types.")
      );
   mHelpStrings.insert(
      make_pair("sethashsize", "sethashsize <size>.....set the size of the transposition tables.")
      );
   mHelpStrings.insert(
      make_pair("setsearch", "setsearch <search>.....set the search to the given type.")
      );
   mHelpStrings.insert(
      make_pair("selfplay", "selfplay <filename> [N]...conduct game (or N games) of self-play.")
      );
   mHelpStrings.insert(
      make_pair("undo", "undo......................undo the last move made.")
      );
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Count
// Description:  Executes a shell command a given number of times
//-------------------------------------------------------------------------
void ConsoleShell::Count(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Execute a command a given number of times." << endl;
   }
   else if (arguments.size() > 2) {
     int count;
     istringstream instring;
     instring.str(arguments[1]);
     if (!(instring >> count)) {
        cout << arguments[1] << " is not a valid number." << endl;
     }
     else if (count <= 0) {
        cout << arguments[1] << " is not a positive value." << endl;
     }
     else {
        // Look up the second argument of the command to find
        // an appropriate input handler method
        InputHandlerMap::iterator itr = mInputHandlers.find(arguments[2]);

        if (itr == mInputHandlers.end()) {
           // No exact match was found;
           // Let's try for a substring match
           itr = find_if(mInputHandlers.begin(),
                         mInputHandlers.end(),
                         substr(arguments[2]));
        }

        if (itr != mInputHandlers.end()) {
           // Call the handler that we've found "count" times
           InputHandlerPtr handlerptr = (*itr).second;
           vector<string> tokens(arguments.begin() + 2, arguments.end());
           for (int i = 0; i < count; i++) {
              (*this.*handlerptr)(tokens);
           }
        }
        else {
           cout << arguments[2] << " is not a valid command." << endl;
        }
     }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Depth
// Description:  Set the depth of the search
//-------------------------------------------------------------------------
void ConsoleShell::Depth(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Set the search depth to the given value." << endl;
   }
   else if (arguments.size() == 2) {
      int depth;
      istringstream instring;

      instring.str(arguments[1]);
      if (!(instring >> depth)) {
         cout << arguments[1] << " is not a valid number." << endl;
      }
      else if (depth <= 0) {
         cout << arguments[1] << " is not a positive value." << endl;
      }
      else {
         mpEngine->SetSearchDepth(depth);
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Display
// Description:  Displays the board
//-------------------------------------------------------------------------
void ConsoleShell::Display(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Print the current board position." << endl;
   }
   else {
      mpEngine->PrintBoard(cout);
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Exit
// Description:  Exits the mShogi Console
//-------------------------------------------------------------------------
void ConsoleShell::Exit(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "exit" << endl;
      cout << "   Return to the previous stream of input, i.e., the one"
           << " that called input." << endl;
   }
   else if (mInputStack.size() == 1) {
      cout << "STDIN is already the keyboard." << endl;
   }
   else {
      // Pop the current input stream off the stack
      // and destroy it if it does not point to STDIN
      istream* streamptr = mInputStack.back();
      mInputStack.pop_back();
      if ( (streamptr != &cin) && (streamptr != 0) ) {
         delete streamptr;
      }
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Games
// Description:  List the game types available to the engine
//-------------------------------------------------------------------------
void ConsoleShell::Games(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "List the game types available to the engine." << endl;
   }
   else if (arguments.size() == 1) {
      vector<string> gamelist;
      mpEngine->GetGameList(gamelist);
      for (unsigned int i = 0; i < gamelist.size(); i++) {
         cout << gamelist[i] << endl;
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }

}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Help
// Description:  Print list of available commands or call the 
//               appropriate input handler to print help 
//               information about a specific command
//-------------------------------------------------------------------------
void ConsoleShell::Help(const vector<string>& arguments)
{
   // Determine whether the user is asking for general
   // help or for help with a specific command
   if (arguments.size() == 2) {
      // Pass this argument list on to the method
      // that handles the current command
      InputHandlerMap::iterator itr = mInputHandlers.find(arguments[1]);
      if (itr != mInputHandlers.end()) {
         // Call the input handler that we've found
         // with a special argument to return help
         // for the current command
         InputHandlerPtr handlerptr = (*itr).second;
         vector<string> tokens;
         
         tokens.push_back("help");
         (*this.*handlerptr)(tokens);
      }
      else {
         // The second argument is not a valid command
         cout << arguments[1] << " is not a valid command." << endl;
      }
   }
   else if (arguments.size() == 1) {
      // User requested general help - print a list of commands
      for (StringMap::iterator itr = mHelpStrings.begin();
           itr != mHelpStrings.end(); itr++)
      {
         cout << (*itr).second << endl;
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  History
// Description:  Print the current game history
//-------------------------------------------------------------------------
void ConsoleShell::History(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Print the current game history." << endl;
   }
   else {
      mpEngine->PrintHistory(cout);
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Input
// Description:  Reads commands from a given file
//-------------------------------------------------------------------------
void ConsoleShell::Input(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "input <filename>" << endl;
      cout << "   Reads commands from <filename>. Make sure the last"
           << " command in the file is exit." << endl;
   }
   else if (arguments.size() == 2) {
      // Try to open the given file for input
      ifstream* streamptr = new ifstream(arguments[1].c_str());

      // Make sure the file was successfully opened
      if ( streamptr->is_open() ) {
         mInputStack.push_back(streamptr);
      }
      else {
         cout << "Unable to open file " << arguments[1] << "." << endl;
         delete streamptr;
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

#ifdef DEBUG
//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Log
// Description:  Toggle the log on and off
//-------------------------------------------------------------------------
void ConsoleShell::Log(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Toggle the log feature on and off." << endl;
   }
   // Explicit value requested
   else if (arguments.size() == 2) {
      if (arguments[1] == "on") {
         mpEngine->SetLog(true);
      }
      else if (arguments[1] == "off") {
         mpEngine->SetLog(false);
      }
      else {
         // The second argument is not a valid command
         cout << arguments[1] << " is not a valid command." << endl;
      }
   }
   // Simply toggle the value
   else if (arguments.size() == 1) {
      mpEngine->SetLog();
   }
   // Error
   else {
      cout << "Invalid number of arguments." << endl;
   }
}
#endif

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  New
// Description:  Start a new game
//-------------------------------------------------------------------------
void ConsoleShell::New(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Start a new session of the current game type "
           << "or of the game type indicated by the argument." << endl;
   }
   else {
      switch ( arguments.size() ) {
      case 2:
         if ( !mpEngine->SetGame(arguments[1]) ) {
            cout << arguments[1] << " is not a valid game." << endl;
            break;
         }
      case 1:
         mpEngine->RestartGame();
         break;
      default:
         cout << "Invalid number of arguments." << endl;
      }
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Quiescence
// Description:  Toggle the quiescence search on and off
//-------------------------------------------------------------------------
void ConsoleShell::Quiescence(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Toggle the quiescence search on and off." << endl;
   }
   // Explicit value requested
   else if (arguments.size() == 2) {
      if (arguments[1] == "on") {
         mpEngine->SetQuiescence(true);
      }
      else if (arguments[1] == "off") {
         mpEngine->SetQuiescence(false);
      }
      else {
         // The second argument is not a valid command
         cout << arguments[1] << " is not a valid command." << endl;
      }
   }
   // Simply toggle the value
   else if (arguments.size() == 1) {
      mpEngine->SetQuiescence();
   }
   // Error
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  QuiescenceDepth
// Description:  Set the depth of the quiescence search
//-------------------------------------------------------------------------
void ConsoleShell::QuiescenceDepth(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Set the quiescence search depth to the given value." << endl;
   }
   else if (arguments.size() == 2) {
      int depth;
      istringstream instring;

      instring.str(arguments[1]);
      if (!(instring >> depth)) {
         cout << arguments[1] << " is not a valid number." << endl;
      }
      else if (depth <= 0) {
         cout << arguments[1] << " is not a positive value." << endl;
      }
      else {
         mpEngine->SetQuiescenceDepth(depth);
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Quit
// Description:  Quits the mShogi Console
//-------------------------------------------------------------------------
void ConsoleShell::Quit(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Quits the application." << endl;
   }
   else {
      cout << "Quitting mShogi..." << endl;
      mFinished = true;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  SaveGame
// Description:  Save game to a file
//-------------------------------------------------------------------------
void ConsoleShell::SaveGame(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Save game to a file." << endl;
   }
   else if (arguments.size() == 2) {
      mpEngine->SaveGame(arguments[1]);
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Search
// Description:  Search the current board position for the best move
//-------------------------------------------------------------------------
void ConsoleShell::Search(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Search the current board position for the best move." << endl;
   }
   // Is the game over?
   else if ( mpEngine->IsGameFinished() ) {
      cout << mpEngine->GameStatusMessage() << endl;
   }
   // The game is still in progress; let's search for the best move
   else {
      string move;
      if ( mpEngine->MakeBestMove(mpEngine->GetSide(), move) )  {
         if (mInputStack.back() == &cin) {
            cout << "Making move: " << move << endl;
            mpEngine->PrintBoard(cout);
            mpEngine->PrintSearchStatistics(cout);
         }
         
         // Is the game now over?
         if ( mpEngine->IsGameFinished() ) {
            cout << mpEngine->GameStatusMessage() << endl;
         }
      }
      else {
         cout << "No move found." << endl;
      }
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Searches
// Description:  List the search types available to the engine
//-------------------------------------------------------------------------
void ConsoleShell::Searches(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "List the search types available to the engine." << endl;
   }
   else if (arguments.size() == 1) {
      vector<string> searchlist;
      mpEngine->GetSearchList(searchlist);
      for (unsigned int i = 0; i < searchlist.size(); i++) {
         cout << searchlist[i] << endl;
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }

}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  SelfPlay
// Description:  Conduct one or more games of self-play
//-------------------------------------------------------------------------
void ConsoleShell::SelfPlay(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Conduct one or more games of self-play, saving the results in a file." << endl;
   }
   else if (arguments.size() <= 3) {
      // Figure out how many games we're supposed to play
      int games;
      if (arguments.size() == 2) {
         games = 1;
      }
      else {
         istringstream instring;
         instring.str(arguments[2]);
         if (!(instring >> games)) {
            cout << arguments[2] << " is not a valid number." << endl;
         }
         else if (games <= 0) {
            cout << arguments[2] << " is not a positive value." << endl;
         }
      }

      // Open a file for recording our game results
      ofstream output(arguments[1].c_str());

      // Play our games
      int count = 1;
      string move;
      while (games > 0) {
         // Start a fresh game
         mpEngine->RestartGame();
      
         // Play game to the finish
         while (! mpEngine->IsGameFinished() ) {
            if (!mpEngine->MakeBestMove(mpEngine->GetSide(), move)) {
               cout << "ERROR: No move was found." << endl;
               return;
            }  
         }
      
         // Record this game
         output << "Game " << count << endl;
         mpEngine->PrintHistory(output);
         mpEngine->PrintResult(output);
         output << endl;
      
         // Next game
         games--;
         count++;
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  SetHashSize
// Description:  Set the size of the transposition tables
//-------------------------------------------------------------------------
void ConsoleShell::SetHashSize(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Set the size of the transposition tables." << endl;
   }
   else if (arguments.size() == 2) {
      int size;
      istringstream instring;

      instring.str(arguments[1]);
      if (!(instring >> size)) {
         cout << arguments[1] << " is not a valid number." << endl;
      }
      else if (size <= 0) {
         cout << arguments[1] << " is not a positive value." << endl;
      }
      else {
         mpEngine->SetHashSize(size);
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  SetSearch
// Description:  Set the search to a given search type
//-------------------------------------------------------------------------
void ConsoleShell::SetSearch(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Set the search to a given search type." << endl;
   }
   else if (arguments.size() == 2) {
      if ( !mpEngine->SetSearch(arguments[1]) ) {
         cout << arguments[1] << " is not a valid search type." << endl;
      }
   }
   else {
      cout << "Invalid number of arguments." << endl;
   }
}

//-------------------------------------------------------------------------
//       Class:  ConsoleShell
//      Method:  Undo
// Description:  Undo the last move made
//-------------------------------------------------------------------------
void ConsoleShell::Undo(const vector<string>& arguments)
{
   if (arguments[0] == "help") {
      cout << "Undo the last move made." << endl;
   }
   else {
      if ( mpEngine->UnmakeMove() )  {
         mpEngine->PrintBoard(cout);
      }
      else {
         cout << "Unable to undo last move." << endl;
      }
   }
}

