////////////////////////////////////////////////////////////////////////////////
// Name:         debug_utils.hpp
// Description:  Debug utilities
// Created:      May 17, 2004 09:45:42 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef DEBUG_UTILS_HPP
#define DEBUG_UTILS_HPP

#include <boost/dynamic_bitset.hpp>

class Notator;
class Move;

extern void pbb(const Bitboard& bitboard);
extern void PrintBitboard(int width, const Bitboard* bptr);
extern void PrintMove(Notator* notator, Move* move);
extern bool keeplog;

#endif // DEBUG_UTILS_HPP

