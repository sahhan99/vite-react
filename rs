// استدعاء مكتبات التشفير الدقيقة والزمن
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

/// هيكل الكتلة الأساسية لنظام سحّان (Sahhan Sovereign Block)
/// "بناءُ (رستٍ) قد شددنا البنا ... حتى تساوى الأُسُّ فوق القصور"
#[derive(Debug, Clone)]
pub struct SahhanBlock {
    pub index: u64,              // ترتيب الكتلة في النظام
    pub timestamp: u128,         // توقيت التوثيق الدقيق
    pub payload: String,         // البيانات المحفوظة (بيانات الهدى)
    pub previous_hash: String,   // ارتباط وثيق بالماضي (جذور صلبة)
    pub hash: String,            // الختم المشفر العَصِيّ على الكسر
}

impl SahhanBlock {
    /// دالة التكوين: إيجاد الكتلة من العدم البرمجي بقوة الأمر
    pub fn new(index: u64, payload: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let mut block = SahhanBlock {
            index,
            timestamp,
            payload,
            previous_hash,
            hash: String::new(), // سيتم توليد الختم لاحقاً
        };

        // تثبيت الختم (Hash) فور التكوين
        block.hash = block.generate_seal();
        block
    }

    /// الخوارزمية السيادية: ختم ابن مسفر 
    /// "مفتاحها بالتشفير قد عقدت ... مغالق عصية للمرور"
    fn generate_seal(&self) -> String {
        let mut hasher = Sha256::new();
        
        // دمج عناصر الكتلة لتكوين البصمة الفريدة
        let data_to_hash = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.payload, self.previous_hash
        );

        hasher.update(data_to_hash.as_bytes());
        let result = hasher.finalize();
        
        // إخراج الختم بصيغة قراءة سداسية عشرية (Hexadecimal)
        format!("{:x}", result)
    }
}

// دالة التشغيل والاختبار الأولى لنظام سحّان
fn main() {
    println!("=== بدء تشغيل نواة نظام سحَّان السيادي ===");
    
    // تكوين الكتلة الأولى (Genesis Block) - نقطة البداية
    let genesis_block = SahhanBlock::new(
        0,
        String::from("بسم الله الأعظم - الكتلة الأولى لنظام سحّان: العدل والسيادة"),
        String::from("0000000000000000000000000000000000000000000000000000000000000000"), // لا جذور قبلها
    );

    println!("الكتلة المستحدثة: {:#?}", genesis_block);
    println!("الختم السيادي (Hash): {}", genesis_block.hash);
    println!("===========================================");
}
