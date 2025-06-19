// import { fetch } from "@tauri-apps/plugin-http";
// import { encode, decode } from "@msgpack/msgpack";

// export async function Call<T = any>(
//   url: string,
//   {
//     method = "POST",
//     data = {},
//     retries = 0,
//     timeout = 6000000,
//     headers = {},
//   }: {
//     method?: "GET" | "POST" | "PUT" | "DELETE";
//     data?: any;
//     retries?: number;
//     timeout?: number;
//     headers?: Record<string, string>;
//   } = {}
// ): Promise<T> {
//   try {
//     const response = await fetch(url, {
//       method,
//       timeout, // milliseconds
//       headers: {
//         "Content-Type": "application/msgpack",
//         Accept: "application/msgpack",
//         ...headers,
//       },
//       body: method !== "GET" ? Array.from(encode(data)) : undefined,
//       responseType: "binary", // we want Uint8Array back
//     });

//     // Convert `number[]` back to Uint8Array
//     const binary = new Uint8Array(response as number[]);
//     return decode(binary) as T;
//   } catch (err: any) {
//     if (retries > 0) {
//       console.warn("Retrying...", retries, err.message);
//       return Call<T>(url, {
//         method,
//         data,
//         retries: retries - 1,
//         timeout,
//         headers,
//       });
//     }
//     throw err;
//   }
// }
