////////////////////////////////////////////////////////////////////////////////
// Name:         tokenize.cpp
// Description:  Implementation file for the function 'tokenize'
// Created:      Apr 19, 2004 10:58:18 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
// ===========================================================================
// Header file includes
// ===========================================================================

// STL header files
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

//----------------------------------------------------------------------------
//  Function:     tokenize
//  Description:  Function that tokenizes a string
//----------------------------------------------------------------------------
void tokenize(const string& str, vector<string>& tokens,
              const string& delimiters)
{
   // Make sure we have no tokens to start with
   tokens.clear();
   
   // Skip delimiters at beginning
   string::size_type lastPos = str.find_first_not_of(delimiters, 0);

   // Find the end of the first token
   string::size_type pos = str.find_first_of(delimiters, lastPos);

   while (string::npos != pos || string::npos != lastPos) {
      // Found a token, add it to the vector.
      tokens.push_back(str.substr(lastPos, pos - lastPos));

      // Skip delimiters
      lastPos = str.find_first_not_of(delimiters, pos);

      // Find the end of the next token
      pos = str.find_first_of(delimiters, lastPos);
   }
}

