////////////////////////////////////////////////////////////////////////////////
// Name:         common.hpp
// Description:  Common type, macro, and constant definitions
// Created:      Apr 19, 2004 10:53:23 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef COMMON_HPP
#define COMMON_HPP

// Program Version
#define VERSION "0.9.1"

// Short Hand
#define Bitboard boost::dynamic_bitset<>

#define BLACK 0
#define WHITE 1

enum {
   HUMAN = 0,
   COMPUTER,
   NETWORK
};

extern const char* colours[2];

// Fundamental types
typedef unsigned char       uint8;
typedef unsigned short     uint16;
typedef unsigned long      uint32;
typedef unsigned long long uint64;


#ifdef DEBUG
#include "debug_utils.hpp"
#endif

#endif // COMMON_HPP
