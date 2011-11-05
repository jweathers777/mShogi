////////////////////////////////////////////////////////////////////////////////
// Name:         ObjectFactory.hpp
// Description:  Template for a class that creates subclasses of a base class
//               based upon some key information
// Created:      10/30/2003 10:38:29 Eastern Standard Time
// Last Updated: $Date: 2004/08/28 02:34:20 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2003 John Weathers
////////////////////////////////////////////////////////////////////////////////

#ifndef __ObjectFactory_HPP__
#define __ObjectFactory_HPP__

// STL header files
#include <vector>
#include <map>

// ===========================================================================
// Abstract functor template for wrapping a subclass's constructor
// ===========================================================================
template <class BaseClass>
class AbstractCreator
{
public:
   virtual BaseClass* operator() ()=0;
};

// ===========================================================================
// Concrete functor template for wrapping a subclass's constructor
// ===========================================================================
template <class BaseClass, class SubClass>
class Creator : public AbstractCreator<BaseClass>
{
public:
   BaseClass* operator() () { return new SubClass(); }
};

// ===========================================================================
// The actual Object Factory template
// ===========================================================================
template <typename KeyType, class BaseClass>
class ObjectFactory
{
   typedef AbstractCreator<BaseClass> ObjectCreator;
   typedef std::map<KeyType, ObjectCreator*> RegistryMap;
private:
   // ==========================================================================
   // Construction and destruction
   // ==========================================================================
   ObjectFactory();

public:
   static ObjectFactory<KeyType, BaseClass>* GetInstance();
   ~ObjectFactory();

   // ==========================================================================
   // Operations
   // ==========================================================================
   BaseClass* Create(const KeyType& key);
   void Register(const KeyType& Key, ObjectCreator* creator);
   
   bool IsRegistered(const KeyType& key);
   void GetKeys(std::vector<KeyType>& keys);
   
private:
   // ==========================================================================
   // Data
   // ==========================================================================
   static ObjectFactory<KeyType, BaseClass>* pInstance;
   RegistryMap mRegistry;
};

// ===========================================================================
// Create the static instance of the Object Factory
// ===========================================================================
template <typename KeyType, class BaseClass>
ObjectFactory<KeyType, BaseClass>*
ObjectFactory<KeyType, BaseClass>::pInstance = 0;

//------------------------------------------------------------------------------
//       Class:  ObjectFactory
//      Method:  ObjectFactory
// Description:  Constructs an instance of an object factory
//------------------------------------------------------------------------------
template <typename KeyType, class BaseClass>
ObjectFactory<KeyType, BaseClass>::ObjectFactory()
{
}

//------------------------------------------------------------------------------
//       Class:  ObjectFactory
//      Method:  ~ObjectFactory
// Description:  Destroys the instance of an object factory
//------------------------------------------------------------------------------
template <typename KeyType, class BaseClass>
ObjectFactory<KeyType, BaseClass>::~ObjectFactory()
{
   if (pInstance) {
      typename RegistryMap::iterator itr;
      ObjectCreator* creator;
      for (itr = pInstance->mRegistry.begin();
           itr != pInstance->mRegistry.end();
           itr++)
      {
         creator = (*itr).second;
         if (creator) {
            delete creator;
         }
      }
      pInstance->mRegistry.clear();

      pInstance = 0;
   }
}

//------------------------------------------------------------------------------
//       Class:  ObjectFactory
//      Method:  GetInstance
// Description:  Returns the instance of the object factory
//------------------------------------------------------------------------------
template <typename KeyType, class BaseClass>
ObjectFactory<KeyType,BaseClass>*
ObjectFactory<KeyType, BaseClass>::GetInstance()
{
   if (!pInstance) {
      pInstance = new ObjectFactory(); 
   }

   return pInstance;
}

//------------------------------------------------------------------------------
//       Class:  ObjectFactory
//      Method:  Create
// Description:  Creates a new subclass instance from a key
//------------------------------------------------------------------------------
template <typename KeyType, class BaseClass>
BaseClass* ObjectFactory<KeyType, BaseClass>::Create(const KeyType& key)
{
   BaseClass* result = 0;
   
   if (pInstance) {
      typename RegistryMap::iterator itr = pInstance->mRegistry.find(key);
      if ( itr != pInstance->mRegistry.end() ) {
         ObjectCreator* creator = (*itr).second;
         result = (*creator)();
      }
   }

   return result;
}

//------------------------------------------------------------------------------
//       Class:  ObjectFactory
//      Method:  Register
// Description:  Registers a class and key with the object factory
//------------------------------------------------------------------------------
template <typename KeyType, class BaseClass>
void 
ObjectFactory<KeyType, BaseClass>::Register(const KeyType& key, 
                                            ObjectCreator* creator)
{
   if ( pInstance && !IsRegistered(key) ) {
      pInstance->mRegistry.insert(make_pair(key, creator));
   }
}

//------------------------------------------------------------------------------
//       Class:  ObjectFactory
//      Method:  IsRegistered
// Description:  Checks to see whether a class is registerd with 
//               the object factory through the given key
//------------------------------------------------------------------------------
template <typename KeyType, class BaseClass>
bool 
ObjectFactory<KeyType, BaseClass>::IsRegistered(const KeyType& key)
{
   return (pInstance && 
           (pInstance->mRegistry.find(key) != pInstance->mRegistry.end())
           );
}

//------------------------------------------------------------------------------
//       Class:  ObjectFactory
//      Method:  GetKeys
// Description:  Fill a vector with the keys registered in the object factory
//------------------------------------------------------------------------------
template <typename KeyType, class BaseClass>
void
ObjectFactory<KeyType, BaseClass>::GetKeys(std::vector<KeyType>& keys)
{
   keys.clear();
   if (pInstance) {
      typename RegistryMap::iterator itr;
      for (itr = pInstance->mRegistry.begin();
           itr != pInstance->mRegistry.end();
           itr++)
      {
         keys.push_back((*itr).first);
      }           
   }
}

#endif

