////////////////////////////////////////////////////////////////////////////////
// Name:         ByValue.hpp
// Description:  Functor template for comparing two objects which have
//               mValue members, such as pieces or moves
// Created:      11/23/2003 02:59:09 PM EST
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef __ByValue_HPP__
#define __ByValue_HPP__

#include <functional>

template <class Type, bool reversed=false>
class ByValue : public std::binary_function<Type, Type, bool>
{
public:
   // =======================================================================
   // Operators
   // =======================================================================
   bool operator()(Type x, Type y) 
   { 
      return (x.mValue < y.mValue);
   }
};

template <class Type>
class ByValue<Type*, false> : public std::binary_function<Type*, Type*, bool>
{
public:
   // =======================================================================
   // Operators
   // =======================================================================
   bool operator()(Type* x, Type* y) 
   { 
      return (x->mValue < y->mValue);
   }
};

template <class Type>
class ByValue<Type, true> : public std::binary_function<Type, Type, bool>
{
public:
   // =======================================================================
   // Operators
   // =======================================================================
   bool operator()(Type x, Type y) 
   { 
      return (x.mValue > y.mValue);
   }
};

template <class Type>
class ByValue<Type*, true> : public std::binary_function<Type*, Type*, bool>
{
public:
   // =======================================================================
   // Operators
   // =======================================================================
   bool operator()(Type* x, Type* y) 
   { 
      return (x->mValue > y->mValue);
   }
};


#endif

