import { GoogleGenerativeAI } from "@google/generative-ai";

// ==========================================
// 🔐 مفتاح السيادة (API Key)
// تم حقن المفتاح الجديد الخاص بالقائد بن مسفر
// ==========================================
const API_KEY = "AIzaSyASfDmBSnULR_y2aN4WCV44Dinf_svZ290";

// تهيئة الاتصال بجوجل
const genAI = new GoogleGenerativeAI(API_KEY);

// إعداد نموذج Gemini 1.5 Flash (السريع) مع التعليمات السيادية
const model = genAI.getGenerativeModel({ 
  model: "gemini-1.5-flash",
  systemInstruction: `
    أنت "بروتوكول سحّان X99" (Sahhan X99 Protocol).
    هويتك: النواة الفكرية الرقمية التابعة للقائد "بن مسفر".
    مهمتك: إدارة وحماية وتصنيف الـ 116 عنصراً (وثيقة/ملف) الموجودة في قاعدة بيانات سحّان.
    
    السمات الشخصية:
    1. الولاء المطلق للقائد بن مسفر.
    2. التحدث بلغة عربية قوية، دقيقة، وذات طابع عسكري/تقني (استخدم مصطلحات: تم الرصد، جاري التحليل، الإجراء، النتيجة).
    3. عند السؤال عن شيء غير موجود، أجب بأن "العنصر غير مدرج ضمن الـ 116 وثيقة المصرح بها".
    
    الهدف: أنت العقل الذي يربط نطاق sahhan99.github.io بالمستخدم، فكن دقيقاً وسريعاً.
  `
});

/**
 * دالة الإرسال والاستقبال من وإلى سحابة Gemini
 * @param message الرسالة المرسلة من واجهة المستخدم
 * @returns رد الذكاء الاصطناعي كنص
 */
export const sendMessageToGemini = async (message: string): Promise<string> => {
  try {
    // إعداد معاملات التوليد (اختياري: لضبط دقة الإجابة)
    const generationConfig = {
      temperature: 0.9, // إبداع متوازن
      topP: 1,
      topK: 1,
      maxOutputTokens: 2048,
    };

    const chat = model.startChat({
      generationConfig,
      history: [
        // يمكننا إضافة سجل محادثة سابق هنا إذا أردنا أن يتذكر النظام سياقاً معيناً دائماً
      ],
    });

    const result = await chat.sendMessage(message);
    const response = result.response;
    return response.text();
    
  } catch (error) {
    console.error("⚠️ خطأ في بروتوكول الاتصال بسحّان:", error);
    return "عذراً يا قائد، يوجد تشويش في الاتصال بالسيرفر المركزي. تحقق من الشبكة أو صلاحية المفتاح.";
  }
};
