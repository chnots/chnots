package org.codeberg.wangzh.chnots

import android.os.Bundle
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

class MainActivity : TauriActivity() {
  // https://github.com/tauri-apps/tauri/discussions/9261#discussioncomment-12702850
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)

//    // Allow content to extend under the system bars
//    WindowCompat.setDecorFitsSystemWindows(window, false)
//
//    // Get the insets controller to manage system UI
//    val windowInsetsController = WindowCompat.getInsetsController(window, window.decorView)
//
//    // Hide both status bar and navigation bar
//    windowInsetsController.apply {
//      hide(WindowInsetsCompat.Type.systemBars())
//      systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
//    }

    // Make both bars transparent
    window.statusBarColor = android.graphics.Color.BLACK
    window.navigationBarColor = android.graphics.Color.BLACK

    // Handle display cutout (notch)
    if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.P) {
      window.attributes.layoutInDisplayCutoutMode =
        android.view.WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
    }
  }
}