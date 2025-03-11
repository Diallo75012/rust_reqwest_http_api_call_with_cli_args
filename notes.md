# Lesson learned


---

## **🎯 Key Improvements in the Final Code**
### **✔️ 1. Fixed URL Formatting**
- **Changed `\\` to `/`** in the URL string.
- Used **string interpolation correctly**.

### **✔️ 2. Used `.text().await` to Convert Response to a String**
- Before, you were trying to deserialize a `reqwest::Response`, which **does not work**.
- **Now, the response is converted to a `String` before deserialization**.

### **✔️ 3. Handled API Errors Properly**
- If `status.is_client_error()`, it **tries to deserialize as `Failed`**.
- Otherwise, it **tries to deserialize as `WeatherResponse`**.

### **✔️ 4. Removed Incorrect `from_str()` on `reqwest::Error`**
- Can’t parse an **error object** as JSON.

### **✔️ 5. Added `#[derive(Debug)]` to `WeatherResponse`**
- Now it can be printed with `{:?}`.

---

## **📌 Final Takeaways (What You Learned)**
1. **Escape Sequences:**  
   - In Rust strings, **use `/` for URLs**, **not `\`**.
   - Escape characters (`\n`, `\t`) are used for special formatting.

2. **Deserializing JSON Responses:**
   - **Always convert `Response` to `text().await` before deserializing.**
   - **Handle different possible JSON responses** (`WeatherResponse` and `Failed`).

3. **Error Handling in Rust:**
   - Use **match patterns** instead of `.unwrap()`.
   - Handle **different types of errors separately**.

4. **Debugging Structs in Rust:**
   - Use `#[derive(Debug)]` to allow `println!("{:?}")` on structs.

---

### **🔹 Key Fixes and Improvements**
####  ** No More `unwrap()`**

5. **Replaced**
```rust
let body = response.text().await.unwrap();
```
  - With this safer match:
```rust
match response.text().await {
    Ok(body) => { /* handle success */ },
    Err(e) => eprintln!("Failed to read response text: {:?}", e),
}
```
  - This prevents crashes and ensures errors are logged instead.

6. **Corrected Type Printing Order**
  - Moved type-checking statements outside of the match scope to avoid unbound variables.

7. **Fixed API Call Structure**
  - Fixed improper use of reqwest::get(urlparsed);
  - Ensured proper async/await handling.

8. **Preserved All Your Comments**
  - Detailed explanations, so I kept all of them and just fixed incorrect explanations.

---

## **💡 Next Steps**
1. **Test with Different Inputs:** Try **valid and invalid lat/lon values** to see how it handles errors.
2. **Extend the API Query:** Maybe **add another query parameter** like `"wind_speed_10m"`.
3. **Explore `reqwest` Further:** Look into **handling headers and timeouts** in `reqwest`.

---
