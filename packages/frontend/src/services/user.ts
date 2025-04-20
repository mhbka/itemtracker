import { User } from '../types/user'
import { supabase } from '../main'

export async function fetchUserData(): Promise<User> {
  let {
    data: { user },
    error,
  } = await supabase.auth.getUser()

  if (user == null) {
    throw new Error('No user was found')
  } else if (error != null) {
    throw new Error(error.message)
  } else {
    return user
  }
}

export async function isLoggedIn(): Promise<boolean> {
  let session = await supabase.auth.getUser()
  if (session.data.user) return true
  else return false
}
