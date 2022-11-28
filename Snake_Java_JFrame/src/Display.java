import java.awt.*;
import java.awt.event.*;
import javax.swing.*;
import java.util.Random;

public class Display extends JPanel implements ActionListener {
  
  boolean running = false;
  Timer timer;
  Random random;
  
  Display() {
    this.setPreferredSize(new Dimension(600, 600));
    this.setBackground(Color.black);
    this.setFocusable(true);
    this.addKeyListener(new MyKeyAdapter());
  }
  public void startGame() {

  }
  public void draw(Graphics g) {
    
    Graphics2D g2D = (Graphics2D) g;
    for(int i=0; i < 600/10; i++) {
      g2D.drawLine(0, i*10, i*10, 600);
      g2D.drawLine(i*10, 0, i*10, 600);
    }

  }
  @Override
  public void actionPerformed(ActionEvent arg0) {
    
    if(running) {
      
    }
    
  }
  public class MyKeyAdapter extends KeyAdapter {
    @Override
    public void keyPressed(KeyEvent e) {

    }
  }
}
