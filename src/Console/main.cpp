////////////////////////////////////////////////////////////////////////////////
// Name:         main.cpp
// Description:  The main function for the mShogi Console program
// Created:      Apr 19, 2004 10:27:55 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// ===========================================================================
// Header file includes
// ===========================================================================

// mShogi header files
#include "common.hpp"
#include "ConsoleShell.hpp"

using namespace std;

int main(int argc, char **argv) 
{
   // Instantiate a console shell for mShogi
   ConsoleShell shell;
   
   // Begin processing user input
   shell.StartShell();   
}
