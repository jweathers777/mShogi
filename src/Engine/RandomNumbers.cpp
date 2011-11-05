////////////////////////////////////////////////////////////////////////////////
// Name:         RandomNumbers.cpp
// Description:  Implementation of custom functions for generating random
//               32-bit and 64-bit numbers
// Created:      June 08, 2004 09:10:34 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#include "common.hpp"

using namespace std;

//---------------------------------------------------------------------------
//  Function:     Random32() 
//  Description:  Function that returns a uniform 32-bit random number.
//                Algorithm take from Crafty, copyright 1996-2001 by 
//                Robert M. Hyatt, Ph.D., Associate Professor of Computer 
//                and Information Sciences, University of Alabama at 
//                Birmingham.
//---------------------------------------------------------------------------
unsigned int Random32()
{
   // Random numbers from Mathematica 2.0
   // SeedRandom = 1;
   // Table[Random[Integer, {0, 2^32 - 1}]
   
   static const unsigned long x[55] = {
      1410651636UL, 3012776752UL, 3497475623UL, 2892145026UL, 1571949714UL,
      3253082284UL, 3489895018UL, 387949491UL, 2597396737UL, 1981903553UL,
      3160251843UL, 129444464UL, 1851443344UL, 4156445905UL, 224604922UL,
      1455067070UL, 3953493484UL, 1460937157UL, 2528362617UL, 317430674UL, 
      3229354360UL, 117491133UL, 832845075UL, 1961600170UL, 1321557429UL,
      747750121UL, 545747446UL, 810476036UL, 503334515UL, 4088144633UL,
      2824216555UL, 3738252341UL, 3493754131UL, 3672533954UL, 29494241UL,
      1180928407UL, 4213624418UL, 33062851UL, 3221315737UL, 1145213552UL,
      2957984897UL, 4078668503UL, 2262661702UL, 65478801UL, 2527208841UL,
      1960622036UL, 315685891UL, 1196037864UL, 804614524UL, 1421733266UL,
      2017105031UL, 3882325900UL, 810735053UL, 384606609UL, 2393861397UL };
      
   static int init = 1;
   static unsigned long y[55];
   static int j, k;
   unsigned long ul;
   
   if (init)
   {
      int i;
      init = 0;
      for (i = 0; i < 55; i++) y[i] = x[i];
      j = 24 - 1;
      k = 55 - 1;
   }

   ul = (y[k] += y[j]);
   if (--j < 0) j = 55 - 1;
   if (--k < 0) k = 55 - 1;
   
   return((unsigned int)ul);
}

//---------------------------------------------------------------------------
//  Function:     Random64() 
//  Description:  Function that returns a uniform 64-bit random number.
//                Algorithm take from Crafty, copyright 1996-2001 by 
//                Robert M. Hyatt, Ph.D., Associate Professor of Computer 
//                and Information Sciences, University of Alabama at 
//                Birmingham.
//---------------------------------------------------------------------------
uint64 Random64()
{
   uint64 result;
   unsigned int r1, r2;
   r1 = Random32();
   r2 = Random32();
   result = r1 | (uint64) r2<<32;

   return result;
}

