////////////////////////////////////////////////////////////////////////////////
// Name:         ConsoleShell.hpp
// Description:  Interface for class that represents a console shell 
//               interface for the mShogi shogi variants engine
// Created:      Apr 19, 2004 10:41:50 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef CONSOLESHELL_HPP
#define CONSOLESHELL_HPP

// Standard C++ header files
#include <iostream>

// STL header files
#include <string>
#include <vector>
#include <map>

// Forward declarations
class Engine;

class ConsoleShell
{
public:
   // ======================================================================
   // Type Definitions
   // ======================================================================
   typedef void (ConsoleShell::*InputHandlerPtr)(const std::vector<std::string>&);
   typedef std::map<std::string, InputHandlerPtr> InputHandlerMap;
   typedef std::map<std::string, std::string> StringMap;
      
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   ConsoleShell();
   ~ConsoleShell();

   // ======================================================================
   // Operations
   // ======================================================================
   void StartShell();
   
private:
   void MainLoop();
   void CreateMaps();

   // ======================================================================
   // Input Handlers
   // ======================================================================
   void Count(const std::vector<std::string>& arguments);
   void Depth(const std::vector<std::string>& arguments);
   void Display(const std::vector<std::string>& arguments);
   void Exit(const std::vector<std::string>& arguments);
   void Games(const std::vector<std::string>& arguments);
   void Help(const std::vector<std::string>& arguments);
   void History(const std::vector<std::string>& arguments);
   void Input(const std::vector<std::string>& arguments);
#ifdef DEBUG
   void Log(const std::vector<std::string>& arguments);
#endif
   void New(const std::vector<std::string>& arguments);
   void Quiescence(const std::vector<std::string>& arguments);
   void QuiescenceDepth(const std::vector<std::string>& arguments);
   void Quit(const std::vector<std::string>& arguments);
   void SaveGame(const std::vector<std::string>& arguments);
   void Search(const std::vector<std::string>& arguments);
   void Searches(const std::vector<std::string>& arguments);
   void SetHashSize(const std::vector<std::string>& arguments);
   void SetSearch(const std::vector<std::string>& arguments);
   void SelfPlay(const std::vector<std::string>& arguments);
   void Undo(const std::vector<std::string>& arguments);
   
   // ======================================================================
   // Classes
   // ======================================================================

   // Functor that returns true if associated string is a substring
   // of the key value of a given value in an InputHandlerMap
   class substr
   {
   public:
      // ===================================================================
      // Construction and destruction
      // ===================================================================
      substr(const std::string& s) { mString = s; }
      
      // ===================================================================
      // Operators
      // ===================================================================
      bool operator()(const InputHandlerMap::value_type& inputValue)
      {
         std::string::size_type index = inputValue.first.find(mString);

         return (index == 0);
      }
      
   private:
      std::string mString;
   };
   
   // ======================================================================
   // Data
   // ======================================================================
   bool     mFinished;   // Set when main loop is finished
   Engine*  mpEngine;    // Game playing engine
   
   std::vector<std::istream*> mInputStack; // Input stream pointer stack
   
   InputHandlerMap mInputHandlers; // Map of commands to input handlers
   
   StringMap mHelpStrings; // Map of commands strings to help strings
};

#endif // CONSOLESHELL_HPP
