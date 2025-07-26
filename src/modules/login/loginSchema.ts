import { z } from 'zod';

export const LoginSchema = z.object({
  username: z.string().min(1, 'Usuario requerido'),
  password_hash: z.string().min(5, 'Mínimo 5 caracteres'),
})

export type LoginData = z.infer<typeof LoginSchema>