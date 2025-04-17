// Metadata for a user (from Supabase).
export interface UserMetadata {
    avatar_url?: string;
    full_name?: string;
    name?: string;
    email?: string;
    picture?: string;
    [key: string]: any;
}

// Information about a user (from Supabase).
export interface User {
  id: string;
  email?: string;
  user_metadata: UserMetadata;
  app_metadata: Record<string, any>;
  aud: string;
  created_at: string;
}