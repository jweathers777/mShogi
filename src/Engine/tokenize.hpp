////////////////////////////////////////////////////////////////////////////////
// Name:         tokenize.hpp
// Description:  Header file for the function 'tokenize'
// Created:      Apr 19, 2004 10:57:07 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

//------------------------------------------------------------------------------
//  Function:     tokenize
//  Description:  Function that tokenizes a string
//------------------------------------------------------------------------------
extern void tokenize(const std::string& str, 
                     std::vector<std::string>& tokens,
                     const std::string& delimiters);
