INSERT INTO rhombus_category VALUES
(1,'pwn','#ef4444'),
(2,'web','#22c55e'),
(3,'crypto','#f59e0b'),
(4,'misc','#a855f7');

INSERT INTO rhombus_author VALUES
(1,'mbund','https://www.gravatar.com/avatar','012345678901234567'),
(2,'mbund2','https://www.gravatar.com/avatar','123456789012345678');

INSERT INTO rhombus_challenge VALUES
(1,'my-first-pwn','desc abc','flag{hi}',1,TRUE,1,0,NULL),
(2,'my-second-pwn','desc def','flag{hi}',1,TRUE,1,0,NULL),
(3,'my-first-web','desc ghi','flag{hi}',2,FALSE,1,1,200),
(4,'wow','desc jkl','flag{hi}',4,TRUE,1,0,NULL),
(5,'rsa','desc mno','flag{hi}',3,FALSE,2,0,NULL);
