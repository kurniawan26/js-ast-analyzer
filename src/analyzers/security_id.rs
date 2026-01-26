// File ini berisi pesan-pesan bahasa Indonesia untuk security analyzer
// Digunakan untuk referensi penggantian pesan

const MESSAGES: &[(&str, &str)] = &[
    // eval
    ("Avoid using eval() - it's a security risk and performance issue",
     "Hindari penggunaan eval() - ini risiko keamanan dan masalah performa"),
    
    // alert
    ("Avoid using alert() - use custom UI for notifications",
     "Hindari penggunaan alert() - gunakan UI kustom untuk notifikasi"),
    
    // Function constructor
    ("Avoid using Function constructor - it's similar to eval() and a security risk",
     "Hindari penggunaan Function constructor - ini mirip eval() dan risiko keamanan"),
    
    // setTimeout/setInterval
    ("Avoid using {} with string argument - use function reference instead",
     "Hindari penggunaan {} dengan argumen string - gunakan referensi fungsi sebagai gantinya"),
    
    // document.write
    ("Avoid document.write() - it clears the entire document",
     "Hindari penggunaan document.write() - ini akan menghapus seluruh dokumen"),
    
    // innerHTML
    ("Using innerHTML can expose you to XSS attacks. Consider using textContent or DOM methods",
     "Penggunaan innerHTML dapat menimbulkan serangan XSS. Pertimbangkan menggunakan textContent atau metode DOM"),
    
    // outerHTML
    ("Using outerHTML can expose you to XSS attacks. Consider using DOM methods instead",
     "Penggunaan outerHTML dapat menimbulkan serangan XSS. Pertimbangkan menggunakan metode DOM sebagai gantinya"),
    
    // console
    ("Remove console.{}() before deploying to production",
     "Hapus console.{}() sebelum deploy ke produksi"),
    
    // hardcoded secrets
    ("Possible hardcoded secret/password detected in variable '{}'",
     "Kemungkinan password/rahasia di-hardcode pada variabel '{}'"),
];
