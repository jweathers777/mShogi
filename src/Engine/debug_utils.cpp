////////////////////////////////////////////////////////////////////////////////
// Name:         debug_utils.cpp
// Description:  Debug utilities
// Created:      May 17, 2004 09:45:42 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifdef DEBUG

#include <iostream>
#include <fstream>
#include <string>
#include "Notator.hpp"
#include "Move.hpp"
#include "common.hpp"

std::ofstream gDebug("mshogi.stdout");

void pbb(const Bitboard& bitboard) {
   PrintBitboard(16, &bitboard);
}

void PrintBitboard(int width, const Bitboard* bptr) 
{
   using namespace std;
   
   int i, j;
   int height = bptr->size() / width;
   cout << endl;
   gDebug << endl;
   for (j = 0; j < height; j++) {
      for (i = width - 1; i >=0; i--) {
         cout << (bptr->test(i+width*j) ? 1 : 0) << ' '; 
         gDebug << (bptr->test(i+width*j) ? 1 : 0) << ' '; 
      }
      cout << endl;
      gDebug << endl;
   }
   gDebug << endl << flush;
}

void PrintMove(Notator* notator, const Move* move)
{
   using namespace std;
   
   string strMove = notator->Notate(move);
   cout << strMove << endl;
   gDebug << strMove << endl;
}

std::ofstream gLog("mshogi.log");
bool keeplog;

#endif
