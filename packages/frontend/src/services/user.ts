import { User } from '../types/user';
import { supabase } from '../main';
import { Post } from '@/types/posts';
import apiClient from './client';

export async function fetchUserData(): Promise<User> {
  let {data: {user}, error} = await supabase.auth.getUser();

  if (user == null) {
    throw new Error("No user was found");
  }
  else if (error != null) {
    throw new Error(error.message);
  }
  else {
    return user;
  }
}

export async function fetchUserPosts(): Promise<Post[]> {
  let response = await apiClient.get('/user/posts');
  return response.data;
  
}

export async function addUserPost(title: string, body: string): Promise<void> {
  await apiClient.post('/user/posts', {title, body});
}