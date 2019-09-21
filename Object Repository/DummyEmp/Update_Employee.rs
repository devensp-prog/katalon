<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update_Employee</name>
   <tag></tag>
   <elementGuidId>4de676d3-55db-4ba0-a4be-1df73c8467a9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;name\&quot;:\&quot;test1\&quot;,\&quot;salary\&quot;:\&quot;1123\&quot;,\&quot;age\&quot;:\&quot;23\&quot;}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/update/${EmpId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>38092ec8-c61b-4341-997a-e33c4d465294</id>
      <masked>false</masked>
      <name>salary</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>eca8c052-099a-4554-83bc-326da1f47668</id>
      <masked>false</masked>
      <name>EmpId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5cc2da97-7ed5-4c89-8a07-7b021c602053</id>
      <masked>false</masked>
      <name>EmpName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3f8ffaee-45a5-4a68-84de-bff2c1935526</id>
      <masked>false</masked>
      <name>age</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
