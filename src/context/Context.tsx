import { createContext, Dispatch, SetStateAction } from 'react'
import { User } from '../models'

type ContextType = {
  user: User,
  setUser: Dispatch<SetStateAction<User>>,
}

const contextSate = {
  user: {
    userid: '',
    name: '',
    role: '',
    exp: 0
  },
  setUser: () => {},
}

export const Context = createContext<ContextType>(contextSate)
