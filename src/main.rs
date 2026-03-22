use calculator::lexical_analysis;
// fn parse(_input: &str) {}
fn main() {
    // 使用正則表達式來去除字串中的空白，方便後續初步的詞法分析
    let regex = regex::Regex::new(r"\s+").unwrap();
    // 模板
    let input = "123 + 24";
    // 輸出原字串，利於除錯
    print!("原字串: \"{}\"\n", input);
    // 判斷字串是否匹配正則表達式，如果匹配就進行替換，否則輸出匹配失敗的訊息
    if regex.is_match(input) {
        // 使用正則表達式替換字串中的空白，這裡使用 replace_all 方法，因為可能有多個空白需要替換，最後再把結果轉成 String 類型的字串，這樣就能方便後續的詞法分析了
        let result = regex.replace_all(input, "").into_owned();
        // 輸出正則表達式替換的結果，利於除錯
        println!("正則表達式替換的結果: \"{}\"", result);
        // 進行詞法分析，這裡暫時只處理數字和加號
        let _ = lexical_analysis(&result);
    } else {
        println!("匹配失敗");
    }
}
